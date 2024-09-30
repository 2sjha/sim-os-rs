#include <stdio.h>
#include <stdbool.h>

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
    char *fname;
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
    unsigned int pid;
    char *fname;
    FILE *fp;
};

typedef struct Spool_List_st Spool_List;
struct Spool_List_st
{
    unsigned int size;
    Spool *sp_arr;
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
int print_init(unsigned int pt);
void print_init_spool(unsigned int pid);
void print_end_spool(unsigned int pid);
void print_print(unsigned int pid, char cpu_print_buf[]);
void print_spool_dump();
void print_terminate();

/*Printer Functions*/
void printer_main(int printer_r, int printer_w);
void printer_init_spool(unsigned int pid);
void printer_end_spool(unsigned int pid, bool pexit);
void printer_dump_spool();
void printer_print(unsigned int pid, char *msg);
void printer_terminate();

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

/*Util Functions*/
void init_readyQ(ReadyQ *rq);
void push_readyQ(ReadyQ *rq, PCB *proc);
void pop_readyQ(ReadyQ *rq);
ReadyQ_Node *rotate_readyQ(ReadyQ *rq);
void dump_readyQ(ReadyQ *rq);

#endif