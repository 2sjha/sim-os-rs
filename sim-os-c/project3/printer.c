#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <string.h>
#include "computer.h"
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
#include <pthread.h>
#include <semaphore.h>

// Reserved CID for Printer instance
unsigned int PRINTER_CID = 0;
extern bool shut_down;

// Simulated Printer vars
unsigned int print_time;
char *paper_fname = "printer.out";
FILE *paper_fp;
char *SPOOL_FNAME = "%d_%d_spool.out";
Spool_List *spools;
pthread_t printer_tid;
unsigned int MAX_SPOOLS_PER_CLIENT = 16;

// Print <-> Printer messages
extern char *SHUTDOWN;
extern char *PRINT_DUMP;
extern char *PID_PRINT_SPOOL_PRINT;
extern char *PID_PRINT_SPOOL_INIT_END;
extern int INIT_OP;
extern int PRINT_OP;
extern int END_OP;

// Printer Manager vars
unsigned int printer_sockfd;
struct sockaddr_in printer_serv_addr;
int printer_serv_addrlen = sizeof(printer_serv_addr);
Comm_Info_List *comms;
Socket_Info soc_info;
sem_t sync_pc;

void print_to_paper(Spool sp, bool pexit)
{
    fprintf(paper_fp, "CID: %d, PID: %d\n", sp.cid, sp.pid);
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

void printer_init_spool(unsigned int cid, unsigned int pid)
{
    char fname[33];
    snprintf(fname, sizeof(fname), SPOOL_FNAME, cid, pid);

    Spool sp;
    sp.cid = cid;
    sp.pid = pid;
    strncpy(sp.fname, fname, sizeof(sp.fname));
    sp.fp = fopen(sp.fname, "w");
    if (sp.fp)
    {
        fprintf(stdout, "[printer.c] (printer_init_spool) : File opened: %s\n", sp.fname);
    }
    else
    {
        fprintf(stderr, "[printer.c] (printer_init_spool) : Couldnt open file: %s\n", sp.fname);
    }

    // Linear search for empty spot in the Spool List for the new process
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].cid == 0 && spools->sp_arr[i].pid == 0)
        {
            spools->sp_arr[i] = sp;
            return;
        }
    }

    fprintf(stderr, "[printer.c] (printer_init_spool) : Printer could not allocate spool for PID: %d.\n", pid);
}

void printer_print(unsigned int cid, unsigned int pid, char *msg)
{
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].cid == cid && spools->sp_arr[i].pid == pid)
        {
            strcat(msg, "\n");
            fputs(msg, spools->sp_arr[i].fp);
            fflush(spools->sp_arr[i].fp);
            return;
        }
    }
}

void printer_end_spool(unsigned int cid, unsigned int pid, bool pexit)
{
    // Linear search process with pid in the PCB List and mark it as empty
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].cid == cid && spools->sp_arr[i].pid == pid)
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

void printer_client_dump_spools(unsigned int cid)
{
    fprintf(stdout, "===========================================\n");
    fprintf(stdout, "           Printer Spools Dump for CID: %d\n", cid);
    fprintf(stdout, "===========================================\n");
    fprintf(stdout, "Index: PID\n");
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].cid == cid && spools->sp_arr[i].pid != 0)
        {
            fprintf(stdout, "%d: CID=%d PID=%d\n", i, spools->sp_arr[i].cid, spools->sp_arr[i].pid);
        }
    }
    fprintf(stdout, "\n");
}

void printer_client_shutdown(unsigned int cid, Comm_Info *comm)
{
    fprintf(stdout, "[printer.c] (printer_client_shutdown) : Client CID=%d shut down started.\n", cid);
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].cid == cid && spools->sp_arr[i].pid != 0)
        {
            printer_end_spool(spools->sp_arr[i].cid, spools->sp_arr[i].pid, false);
        }
    }

    close(comm->sockfd);
    comm->soc_closed = true;
    free(comm->msg_q);
    comm->msg_count = 0;

    fprintf(stdout, "[printer.c] (printer_client_shutdown) : Client CID=%d shut down completed.\n", cid);
}

void printer_handle_msg(Comm_Info *comm, char *buf)
{
    int pid = -1;
    int cid = -1;
    int op = -1;
    char *big_msg = strdup(buf);
    char *buf_i = big_msg;
    char msg[65] = "";
    char cpu_print_msg[33] = "";

    while (*buf_i != '\0')
    {
        int i = 0;
        while (*buf_i != '\n' && *buf_i != '\0')
        {
            msg[i] = *buf_i;
            i++;
            buf_i++;
        }

        if (*buf_i != '\0')
        {
            buf_i++;
        }

        strcat(msg, "\n");
        pid = -1;
        cid = -1;
        op = -1;

        sscanf(msg, PID_PRINT_SPOOL_INIT_END, &cid, &pid, &op);
        if (pid > 0 && op == 0)
        {
            printer_init_spool(cid, pid);
            continue;
        }

        sscanf(msg, PID_PRINT_SPOOL_PRINT, &cid, &pid, &op, cpu_print_msg);
        if (pid > 0 && op == 1)
        {
            printer_print(cid, pid, cpu_print_msg);
            continue;
        }

        sscanf(msg, PID_PRINT_SPOOL_INIT_END, &cid, &pid, &op);
        if (pid > 0 && op == 2)
        {
            printer_end_spool(cid, pid, true);
            continue;
        }

        sscanf(msg, SHUTDOWN, &cid);
        if (cid > 0)
        {
            printer_client_shutdown(cid, comm);
            continue;
        }

        sscanf(msg, PRINT_DUMP, &cid);
        if (cid > 0)
        {
            printer_client_dump_spools(cid);
            continue;
        }
    }
    free(big_msg);
}

void *communicator(void *args)
{
    char printer_buf[2000];
    Comm_Info *comm_info = (Comm_Info *)args;
    comm_info->msg_count = 0;
    comm_info->msg_q = calloc(comm_info->mqs, sizeof(char *));
    sem_init(&comm_info->msg_qfull, 0, comm_info->mqs);
    sem_init(&comm_info->msg_qmutex, 0, 1);

    while (!shut_down)
    {
        // Semaphores to read from connection queue
        sem_wait(&soc_info.conn_qempty);
        sem_wait(&soc_info.conn_qmutex);
        comm_info->sockfd = soc_info.conn_sockets[--soc_info.conn_soc_i];
        sem_post(&soc_info.conn_qfull);
        sem_post(&soc_info.conn_qmutex);
        comm_info->soc_closed = false;

        while (!comm_info->soc_closed)
        {
            memset(printer_buf, 0, sizeof(printer_buf));
            int printer_sock_recv_count = read(comm_info->sockfd, printer_buf, sizeof(printer_buf));
            if (printer_sock_recv_count == 0)
            {
                // Sleep and check again later for messages
                usleep(1000);
            }
            else if (printer_sock_recv_count < 0)
            {
                fprintf(stderr, "[printer.c] (communicator) : Socket receive error.\n");
                break;
            }
            else
            {
                sem_wait(&(comm_info->msg_qfull));
                sem_wait(&(comm_info->msg_qmutex));
                comm_info->msg_q[comm_info->msg_count] = strdup(printer_buf);
                comm_info->msg_count++;
                sem_post(&(comm_info->msg_qmutex));
                sem_post(&sync_pc);
            }
        }
    }
}

void printer_init(unsigned int pt, unsigned int n_comms)
{
    // Init simulated paper printer
    spools = calloc(1, sizeof(Spool_List));
    spools->size = n_comms * MAX_SPOOLS_PER_CLIENT;
    spools->sp_arr = calloc(spools->size, sizeof(Spool));
    print_time = pt;
    paper_fp = fopen(paper_fname, "w");
    if (paper_fp == NULL)
    {
        fprintf(stderr, "[printer.c] (printer_init) : Couldnt open Paper file: %s.\n", paper_fname);
        exit(1);
    }

    sem_init(&sync_pc, 0, 0); // init value may need to be changed
    fprintf(stdout, "[printer.c] (printer_init) : Printer Ready.\n");
}

void printer_manager_init(char *pm_ip, unsigned int pm_port,
                          unsigned int n_comms, unsigned int conn_qsize, unsigned int msg_qsize)
{
    // Init communicators & semaphore on connection queue for the communicators
    soc_info.conn_soc_i = 0;
    soc_info.conn_qs = conn_qsize;
    soc_info.conn_sockets = calloc(conn_qsize, sizeof(int));
    sem_init(&soc_info.conn_qfull, 0, conn_qsize);
    sem_init(&soc_info.conn_qempty, 0, 0);
    sem_init(&soc_info.conn_qmutex, 0, 1);

    comms = calloc(1, sizeof(Comm_Info_List));
    comms->size = n_comms;
    comms->comms_arr = calloc(n_comms, sizeof(Comm_Info));
    memset(comms->comms_arr, 0, n_comms * sizeof(Comm_Info));

    for (int i = 0; i < comms->size; ++i)
    {
        comms->comms_arr[i].mqs = msg_qsize;
        comms->comms_arr[i].t_no = i + 1;
        pthread_create(&(comms->comms_arr[i].comm_tid), NULL, communicator, &comms->comms_arr[i]);
    }

    // Init printer manager socket server
    int printer_sock_status, printer_sock_send_count, printer_sock_recv_count;

    printer_sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (printer_sockfd < 0)
    {
        fprintf(stderr, "[printer.c] (printer_manager_init) : Socket creation error.\n");
        exit(1);
    }

    bzero((char *)&printer_serv_addr, sizeof(printer_serv_addr));
    printer_serv_addr.sin_family = AF_INET;
    printer_serv_addr.sin_addr.s_addr = inet_addr(pm_ip);
    printer_serv_addr.sin_port = htons(pm_port);

    printer_sock_status = bind(printer_sockfd, (struct sockaddr *)&printer_serv_addr, sizeof(printer_serv_addr));
    if (printer_sock_status < 0)
    {
        fprintf(stderr, "[printer.c] (printer_manager_init) : Socket connection error.\n");
        exit(1);
    }

    printer_sock_status = listen(printer_sockfd, n_comms);
    if (printer_sock_status < 0)
    {
        perror("listen");
        exit(EXIT_FAILURE);
    }

    fprintf(stdout, "[printer.c] (printer_manager_init) : Printer server Ready.\n");
}

void printer_manager_all_spools_dump()
{
    fprintf(stdout, "===========================================\n");
    fprintf(stdout, "           Printer All Spools Dump\n");
    fprintf(stdout, "===========================================\n");
    fprintf(stdout, "Index: PID\n");
    for (int i = 0; i < spools->size; ++i)
    {
        if (spools->sp_arr[i].cid != 0 && spools->sp_arr[i].pid != 0)
        {
            fprintf(stdout, "%d: CID=%d PID=%d\n", i, spools->sp_arr[i].cid, spools->sp_arr[i].pid);
        }
    }
    fprintf(stdout, "\n");
}

void printer_manager_terminate()
{
    fclose(paper_fp);

    for (int i = 0; i < comms->size; ++i)
    {
        close(comms->comms_arr[i].sockfd);
        sem_destroy(&(comms->comms_arr[i].msg_qfull));
        sem_destroy(&(comms->comms_arr[i].msg_qmutex));
    }

    free(comms);
    shutdown(printer_sockfd, SHUT_RDWR);
}

void *printer_main(void *arg)
{
    char *buf;
    while (!shut_down)
    {
        sem_wait(&sync_pc);

        for (int i = 0; i < comms->size; ++i)
        {
            if (comms->comms_arr[i].msg_count != 0)
            {
                // Semaphores to read from communicator message queues
                sem_wait(&comms->comms_arr[i].msg_qmutex);

                for (int j = 0; j < comms->comms_arr[i].msg_count; ++j)
                {
                    buf = comms->comms_arr[i].msg_q[j];
                    printer_handle_msg(&(comms->comms_arr[i]), buf);
                    free(buf);
                    sem_post(&comms->comms_arr[i].msg_qfull);
                }
                comms->comms_arr[i].msg_count = 0;
                sem_post(&comms->comms_arr[i].msg_qmutex);
            }
        }
    }
}

void printer_manager_main()
{
    pthread_create(&printer_tid, NULL, printer_main, NULL);

    while (!shut_down)
    {
        // blocking
        int printer_new_socket = accept(printer_sockfd, (struct sockaddr *)&printer_serv_addr, (socklen_t *)&printer_serv_addrlen);
        if (printer_new_socket < 0)
        {
            if (!shut_down)
            {
                perror("accept");
                exit(EXIT_FAILURE);
            }
        }
        else
        {
            sem_wait(&soc_info.conn_qfull);
            sem_wait(&soc_info.conn_qmutex);
            soc_info.conn_sockets[soc_info.conn_soc_i++] = printer_new_socket;
            sem_post(&soc_info.conn_qmutex);
            sem_post(&soc_info.conn_qempty);
        }
    }
}