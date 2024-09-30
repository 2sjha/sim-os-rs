#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <signal.h>
#include "computer.h"

// Print <-> Printer pipe messages
char *PRINT_ACK = "ACK";
char *PRINT_NO_ACK = "NO_ACK";
char *SHUTDOWN = "SHUTDOWN";
char *PRINT_DUMP = "DUMP";
char *PID_PRINT_SPOOL_INIT_END = "PID: %d %d\n";   // Init Code: 0
char *PID_PRINT_SPOOL_PRINT = "PID: %d %d | %s\n"; // Print Code: 1
int INIT_OP = 0;
int PRINT_OP = 1;
int END_OP = 2;

// We assume that CPU->Print and Print<->Printer Buffer is 32 chars long
char cpu_print_buf[33];
char print_buf[33];
int fds1[2], fds2[2];
int printer_r, printer_w, print_r, print_w;
unsigned int child_pid;
unsigned int print_time;
extern PCB_List *pcbl;

void kill_child_process()
{
    kill(child_pid, SIGTERM);

    bool died = false;

    int status;
    pid_t id;

    if (waitpid(child_pid, &status, WNOHANG) == child_pid)
        died = true;

    if (!died)
        kill(child_pid, SIGKILL);
}

int print_init(unsigned int pt)
{
    // Setup Pipes for Print and Printer
    print_time = pt;
    pipe(fds1);
    pipe(fds2);
    printer_r = fds1[0];
    printer_w = fds2[1];
    print_r = fds2[0];
    print_w = fds1[1];

    child_pid = fork();
    if (child_pid == 0)
    {
        // Run printer_main in the child process.
        // Since thats an infinite loop,
        // We never return and proceed our normal flow.
        // And this process will also be shut down when parent process shuts down
        close(print_r);
        close(print_w);
        printer_main(printer_r, printer_w);
    }
    else
    {
        // Wait for ACK
        close(printer_r);
        close(printer_w);
        int to_read;
        int fail_count = 0;
        while (1)
        {
            to_read = read(print_r, print_buf, sizeof(print_buf));
            if (to_read > 0 && !strcmp(print_buf, PRINT_ACK))
            {
                printf("[print.c] (print_init) : Received ACK from printer child process.\n");
                break;
            }
            else if (to_read > 0 && !strcmp(print_buf, PRINT_NO_ACK))
            {
                printf("[print.c] (print_init) : Received NO_ACK from printer child process. Killing it now.\n");
                kill_child_process();
                break;
            }
            fail_count++;

            if (fail_count > 1000)
            {
                kill_child_process();
            }
        }
    }
    return child_pid;
}

void print_init_spool(unsigned int pid)
{
    memset(print_buf, 0, sizeof(print_buf));
    snprintf(print_buf, 33, PID_PRINT_SPOOL_INIT_END, pid, INIT_OP);
    write(print_w, print_buf, sizeof(print_buf));
}

void print_print(unsigned int pid, char *cpu_print_buf)
{
    memset(print_buf, 0, sizeof(print_buf));
    snprintf(print_buf, 33, PID_PRINT_SPOOL_PRINT, pid, PRINT_OP, cpu_print_buf);
    write(print_w, print_buf, sizeof(print_buf));
}

void print_end_spool(unsigned int pid)
{
    memset(print_buf, 0, sizeof(print_buf));
    snprintf(print_buf, 33, PID_PRINT_SPOOL_INIT_END, pid, END_OP);
    write(print_w, print_buf, sizeof(print_buf));
}

void print_spool_dump()
{
    memset(print_buf, 0, sizeof(print_buf));
    strcpy(print_buf, PRINT_DUMP);
    write(print_w, print_buf, sizeof(print_buf));
}

void print_terminate()
{
    printf("[print.c] (print_terminate) : Print shut down started.\n");
    memset(print_buf, 0, sizeof(print_buf));
    strcpy(print_buf, SHUTDOWN);
    write(print_w, print_buf, sizeof(print_buf));

    close(print_r);
    close(print_w);

    printf("[print.c] (print_terminate) : Print shut down completed.\n");
}