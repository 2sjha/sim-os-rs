#include <stdio.h>
#include <stdbool.h>
#include <pthread.h>
#include <semaphore.h>

#ifndef COMPUTER_H
#define COMPUTER_H

/* Simulated Register*/
typedef struct Register_st Register;
struct Register_st
{
    int reg_val;
};

/*Simulated RegisterFile*/
typedef struct Register_File_st Register_File;
struct Register_File_st
{
    Register PC;
    Register IR0;
    Register IR1;
    Register AC;
    Register MAR;
    Register MBR;
    Register BASE;
};

/*Simulated Memory*/
typedef struct Memory_st Memory;
struct Memory_st
{
    unsigned int size;
    int *mem_arr;
};

/*PCB*/
typedef struct PCB_st PCB;
struct PCB_st
{
    Register_File reg_state;
    unsigned int pid;
    char prog_fname[33];
};

/*PCB List Data Structure*/
typedef struct PCB_List_st PCB_List;
struct PCB_List_st
{
    unsigned int size;
    PCB *pcb_arr;
};

typedef struct ReadyQ_Node_st ReadyQ_Node;
struct ReadyQ_Node_st
{
    PCB *proc;
    ReadyQ_Node *prev;
    ReadyQ_Node *next;
};

typedef struct ReadyQ_st ReadyQ;
struct ReadyQ_st
{
    int size;
    ReadyQ_Node *head;
    ReadyQ_Node *tail;
};

typedef struct Spool_st Spool;
struct Spool_st
{
    unsigned int cid;
    unsigned int pid;
    char fname[33];
    FILE *fp;
};

typedef struct Spool_List_st Spool_List;
struct Spool_List_st
{
    unsigned int size;
    Spool *sp_arr;
};

typedef struct Comm_Info_st Comm_Info;
struct Comm_Info_st
{
    pthread_t comm_tid;
    unsigned int t_no;
    unsigned int mqs;
    char **msg_q;
    unsigned int msg_count;
    int sockfd;
    bool soc_closed;
    sem_t msg_qfull, msg_qmutex;
};

typedef struct Comm_Info_List_st Comm_Info_List;
struct Comm_Info_List_st
{
    unsigned int size;
    Comm_Info *comms_arr;
};

typedef struct Socket_Info_st Socket_Info;
struct Socket_Info_st
{
    int *conn_sockets;
    int conn_soc_i;
    unsigned int conn_qs;
    sem_t conn_qfull, conn_qempty, conn_qmutex;
};

/*Memory Functions*/
void mem_init(unsigned int M);
void mem_read();
void mem_write();
void mem_dump();

/*CPU Functions*/
int cpu_operation();
void cpu_reg_dump();

/*Print Functions*/
int print_init(char *pm_ip, unsigned int pm_port);
void print_init_spool(unsigned int pid);
void print_end_spool(unsigned int pid);
void print_print(unsigned int pid, char cpu_print_buf[]);
void print_spool_dump();
void print_terminate();

/*Printer Functions*/
void printer_manager_init(char *pm_ip, unsigned int pm_port, unsigned int n_comms, unsigned int conn_qsize, unsigned int msg_qsize);
void printer_init(unsigned int pt, unsigned int n_comms);
void printer_manager_main();
void printer_manager_all_spools_dump();
void printer_manager_terminate();

/*Scheduler Functions*/
void process_dump_PCBs();
void process_dump_readyQ();
void process_init();
void process_submit(unsigned int p_base, char *p_fname);
void process_execute();

/*Shell Functions*/
void shell_instruction(int cmd);
void shell_load_prog(FILE *prog_f, int p_addr);
void *shell_operation(void *arg);
void *printer_shell_operation(void *arg);

/*Util Functions*/
void init_readyQ(ReadyQ *rq);
void push_readyQ(ReadyQ *rq, PCB *proc);
void pop_readyQ(ReadyQ *rq);
ReadyQ_Node *rotate_readyQ(ReadyQ *rq);
void dump_readyQ(ReadyQ *rq);
unsigned int parse_int(char *num);

#endif