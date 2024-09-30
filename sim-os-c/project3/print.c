#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "computer.h"
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
#include <unistd.h>

// Print <-> Printer messages
char *SHUTDOWN = "CID: %d - SHUTDOWN";
char *PRINT_DUMP = "CID: %d - DUMP";
char *PID_PRINT_SPOOL_INIT_END = "CID: %d PID: %d - OP: %d\n";   // Init : 0 & End: 2
char *PID_PRINT_SPOOL_PRINT = "CID: %d PID: %d - OP: %d | %s\n"; // Print Code: 1
int INIT_OP = 0;
int PRINT_OP = 1;
int END_OP = 2;

char print_buf[65];
unsigned int print_sockfd;
int printer_connected = 0;
int print_sock_send_count;
extern unsigned int cid;

int print_init(char *pm_ip, unsigned int pm_port)
{
    int print_sock_status;
    struct sockaddr_in printer_serv_addr;
    print_sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (print_sockfd < 0)
    {
        fprintf(stderr, "[print.c] (print_init) : Socket creation error.\n");
        return -1;
    }

    bzero((char *)&printer_serv_addr, sizeof(printer_serv_addr));
    printer_serv_addr.sin_family = AF_INET;
    printer_serv_addr.sin_addr.s_addr = inet_addr(pm_ip);
    printer_serv_addr.sin_port = htons(pm_port);

    print_sock_status = connect(print_sockfd, (struct sockaddr *)&printer_serv_addr, sizeof(printer_serv_addr));
    if (print_sock_status < 0)
    {
        fprintf(stderr, "[print.c] (print_init) : Socket connection error.\n");
        return -1;
    }
    else
    {
        printer_connected = 1;
        fprintf(stdout, "[print.c] (print_init) : Printer connection established.\n");
    }
}

int send_msg_printer()
{
    if (!printer_connected)
    {
        fprintf(stderr, "[print.c] (send_msg_printer) : Printer not connected.\n");
        return -1;
    }

    print_sock_send_count = send(print_sockfd, print_buf, strlen(print_buf), 0);
    if (print_sock_send_count < 0)
    {
        fprintf(stderr, "[print.c] (send_msg_printer) : Socket send error.\n");
        return -1;
    }
    return print_sock_send_count;
}

void print_init_spool(unsigned int pid)
{
    memset(print_buf, 0, sizeof(print_buf));
    snprintf(print_buf, sizeof(print_buf), PID_PRINT_SPOOL_INIT_END, cid, pid, INIT_OP);
    send_msg_printer();
}

void print_print(unsigned int pid, char *cpu_print_buf)
{
    memset(print_buf, 0, sizeof(print_buf));
    snprintf(print_buf, sizeof(print_buf), PID_PRINT_SPOOL_PRINT, cid, pid, PRINT_OP, cpu_print_buf);
    send_msg_printer();
}

void print_end_spool(unsigned int pid)
{
    memset(print_buf, 0, sizeof(print_buf));
    snprintf(print_buf, sizeof(print_buf), PID_PRINT_SPOOL_INIT_END, cid, pid, END_OP);
    send_msg_printer();
}

void print_spool_dump()
{
    memset(print_buf, 0, sizeof(print_buf));
    snprintf(print_buf, sizeof(print_buf), PRINT_DUMP, cid);
    send_msg_printer();
}

void print_terminate()
{
    fprintf(stdout, "[print.c] (print_terminate) : Print shut down started.\n");

    memset(print_buf, 0, sizeof(print_buf));
    snprintf(print_buf, sizeof(print_buf), SHUTDOWN, cid);
    send_msg_printer();

    close(print_sockfd);

    fprintf(stdout, "[print.c] (print_terminate) : Print shut down completed.\n");
}