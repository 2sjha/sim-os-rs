#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <string.h>
#include "computer.h"

char printer_buf[1000];
char *paper_fname = "printer.out";
FILE *paper_fp;
char *SPOOL_FNAME = "%d_spool.out";
Spool_List *spools;

extern unsigned int print_time;
extern bool shut_down;
extern PCB_List *pcbl;
extern int fds1[2], fds2[2];
extern int printer_r, printer_w;
extern char *PRINT_ACK;
extern char *PRINT_NO_ACK;
extern char *SHUTDOWN;
extern char *PRINT_DUMP;
extern char *PID_PRINT_SPOOL_PRINT;
extern char *PID_PRINT_SPOOL_INIT_END;
extern int INIT_OP;
extern int PRINT_OP;
extern int END_OP;

void printer_init()
{
    spools = calloc(1, sizeof(Spool_List));
    spools->size = pcbl->size;
    spools->sp_arr = calloc(pcbl->size, sizeof(Spool));

    paper_fp = fopen(paper_fname, "w");
    if (paper_fp == NULL)
    {
        // Send NO_ACK
        strcpy(printer_buf, PRINT_NO_ACK);
        write(printer_w, printer_buf, sizeof(printer_buf));
        return;
    }

    // Send ACK
    strcpy(printer_buf, PRINT_ACK);
    write(printer_w, printer_buf, sizeof(printer_buf));
}

void printer_init_spool(unsigned int pid)
{
    char *fname = malloc(16 * sizeof(char));
    sprintf(fname, SPOOL_FNAME, pid);

    Spool sp;
    sp.pid = pid;
    sp.fname = fname;
    sp.fp = fopen(sp.fname, "w");
    if (sp.fp)
    {
        fprintf(stdout, "File opened: %s\n", sp.fname);
    }
    else
    {
        fprintf(stderr, "Couldnt open file: %s\n", sp.fname);
    }

    // Linear search for empty spot in the Spool List for the new process
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].pid == 0)
        {
            spools->sp_arr[i] = sp;
            return;
        }
    }

    fprintf(stdout, "[printer.c] (printer_init_spool) : Printer could not allocate spool for PID: %d.\n", pid);
}

void printer_print(unsigned int pid, char *msg)
{
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].pid == pid)
        {
            strcat(msg, "\n");
            fputs(msg, spools->sp_arr[i].fp);
            fflush(spools->sp_arr[i].fp);
            return;
        }
    }
}

void print_to_paper(Spool sp, bool pexit)
{
    fprintf(paper_fp, "PID: %d\n", sp.pid);
    fputs("----------------------------------------------------------\n", paper_fp);
    fflush(paper_fp);

    char c = fgetc(sp.fp);
    while (c != EOF)
    {
        fputc(c, paper_fp);
        if (c == '\n')
        {
            usleep(print_time);
        }
        c = fgetc(sp.fp);
    }
    if (!pexit)
    {
        fputs("---------- Process terminated due to shutdown ----------\n\n", paper_fp);
    }
    else
    {
        fputc('\n', paper_fp);
    }
    fflush(paper_fp);
}

void printer_end_spool(unsigned int pid, bool pexit)
{
    // Linear search process with pid in the PCB List and mark it as empty
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].pid == pid)
        {
            fclose(spools->sp_arr[i].fp);
            spools->sp_arr[i].fp = fopen(spools->sp_arr[i].fname, "r");
            print_to_paper(spools->sp_arr[i], pexit);
            fclose(spools->sp_arr[i].fp);
            remove(spools->sp_arr[i].fname);
            memset(&(spools->sp_arr[i]), 0, sizeof(Spool));
            return;
        }
    }
}

void printer_dump_spools()
{
    fprintf(stdout, "===========================================\n");
    fprintf(stdout, "           Printer Dump\n");
    fprintf(stdout, "===========================================\n");
    fprintf(stdout, "Index: PID\n");
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].pid != 0)
        {
            fprintf(stdout, "%d: %d\n", i, spools->sp_arr[i].pid);
        }
    }
    fprintf(stdout, "\n");
}

void printer_handle_pipe_msg()
{
    int pid = -1;
    int op = -1;
    char *tmp_buf;
    tmp_buf = printer_buf;
    char msg[33] = "";
    char print_msg[33] = "";
    strcpy(tmp_buf, printer_buf);

    while (*tmp_buf != '\0')
    {
        int i = 0;
        while (*tmp_buf != '\n')
        {
            msg[i] = *tmp_buf;
            i++;
            tmp_buf++;
        }
        tmp_buf++;

        strcat(msg, "\n");

        if (!strcmp(printer_buf, SHUTDOWN))
        {
            printer_terminate();
            exit(0);
        }
        else if (!strcmp(printer_buf, PRINT_DUMP))
        {
            printer_dump_spools();
            continue;
        }

        sscanf(msg, PID_PRINT_SPOOL_INIT_END, &pid, &op);
        if (pid > 0 && op == 0)
        {
            printer_init_spool(pid);
            continue;
        }

        sscanf(msg, PID_PRINT_SPOOL_PRINT, &pid, &op, print_msg);
        if (pid > 0 && op == 1)
        {
            printer_print(pid, print_msg);
            continue;
        }

        sscanf(msg, PID_PRINT_SPOOL_INIT_END, &pid, &op);
        if (pid > 0 && op == 2)
        {
            printer_end_spool(pid, true);
            continue;
        }
    }
}

void printer_terminate()
{
    fprintf(stdout, "[printer.c] (printer_terminate) : Printer shut down started.\n");
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].pid != 0)
        {
            printer_end_spool(spools->sp_arr[i].pid, false);
        }
    }

    shut_down = true;
    close(printer_r);
    close(printer_w);
    fclose(paper_fp);
    fprintf(stdout, "[printer.c] (printer_terminate) : Printer shut down completed.\n");
}

void printer_main(int printer_r, int printer_w)
{
    printer_init();
    if (paper_fp == NULL)
    {
        return;
    }

    int to_read;
    while (1)
    {
        to_read = read(printer_r, printer_buf, sizeof(printer_buf));
        if (to_read > 0)
        {
            printer_handle_pipe_msg();
        }
    }

    return;
}