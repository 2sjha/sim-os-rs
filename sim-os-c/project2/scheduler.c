#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <unistd.h>
#include "computer.h"

PCB_List *pcbl;
ReadyQ *rq;
PCB *proc_idle;
unsigned int pid_count = 1;
unsigned int time_quantum;
unsigned int curr_pid = 1;
extern Memory mem;
extern Register_File regs;
extern bool shut_down;

void process_init_PCBs()
{
    pcbl = malloc(sizeof(PCB_List));
    // We assume that the average user program
    // is about 20 instructions long, so 40 words.
    // Thus, we can have (mem_size/avg_prog_size) processes
    // active in memory at a time.
    unsigned int avg_prog_size = 40;
    pcbl->size = mem.size / avg_prog_size;
    pcbl->pcb_arr = (PCB *)calloc(pcbl->size, sizeof(PCB));
}

PCB *process_init_PCB(char *fname, unsigned int base)
{
    PCB process;
    Register_File reg_state;
    reg_state.BASE.reg_val = base;
    reg_state.PC.reg_val = 0;
    reg_state.IR0.reg_val = -1;
    reg_state.IR1.reg_val = -1;
    reg_state.AC.reg_val = 0;
    reg_state.MAR.reg_val = 0;
    reg_state.MBR.reg_val = 0;

    process.pid = pid_count;
    process.fname = fname;
    process.reg_state = reg_state;
    pid_count++;

    // Linear search for empty spot in the PCB List for the new process
    for (int i = 0; i < pcbl->size; ++i)
    {
        if (pcbl->pcb_arr[i].pid == 0)
        {
            pcbl->pcb_arr[i] = process;
            return &(pcbl->pcb_arr[i]);
        }
    }

    printf("[scheduler.c] (process_init_PCB) : Scheduler could not allocate process (fname = %s) in the PCB List.\n", fname);
    return NULL;
}

void process_dispose_PCB(unsigned int pid)
{
    // Linear search process with pid in the PCB List and mark it as empty
    for (int i = 0; i < pcbl->size; ++i)
    {
        if (pcbl->pcb_arr[i].pid == pid)
        {
            memset(&(pcbl->pcb_arr[i]), 0, sizeof(PCB));
            return;
        }
    }
}

void process_dump_PCBs()
{
    printf("===========================================\n");
    printf("           PCB Dump\n");
    printf("===========================================\n");
    printf("Index: [ Filename:XXXX, PID:#, BASE:#, PC:#, IR0:#, IR1:#, AC:#, MAR:#, MBR:# ]\n");
    for (int i = 0; i < pcbl->size; ++i)
    {
        if (pcbl->pcb_arr[i].pid != 0)
        {
            printf("%d: [ Filename:%s,", i, pcbl->pcb_arr[i].fname);
            printf(" PID:%d, BASE:%d,", pcbl->pcb_arr[i].pid, pcbl->pcb_arr[i].reg_state.BASE.reg_val);
            printf(" PC:%d, IR0:%d,", pcbl->pcb_arr[i].reg_state.PC.reg_val, pcbl->pcb_arr[i].reg_state.IR0.reg_val);
            printf(" IR1:%d, AC:%d,", pcbl->pcb_arr[i].reg_state.IR1.reg_val, pcbl->pcb_arr[i].reg_state.AC.reg_val);
            printf(" MAR:%d, MBR:%d ]\n", pcbl->pcb_arr[i].reg_state.MAR.reg_val, pcbl->pcb_arr[i].reg_state.MBR.reg_val);
        }
    }
    printf("\n");
}

void process_init_readyQ()
{
    rq = malloc(sizeof(ReadyQ));
    init_readyQ(rq);
}

void process_insert_readyQ(PCB *proc)
{
    push_readyQ(rq, proc);
}

PCB *process_fetch_readyQ()
{
    if (rq->size == 0)
    {
        return NULL;
    }

    return rq->head->next->proc;
}

void process_dump_readyQ()
{
    printf("===========================================\n");
    printf("           ReadyQ Dump. RQ Size: %d\n", rq->size);
    printf("===========================================\n");
    printf("Index: Process ID\n");

    dump_readyQ(rq);
    printf("\n");
}

void process_context_switch(PCB *proc_in, PCB *proc_out)
{
    if (proc_out)
    {
        proc_out->reg_state = regs;
    }

    if (proc_in)
    {
        regs = proc_in->reg_state;
    }
}

void process_init_idle()
{
    FILE *prog_idle = fopen("prog_idle", "r");
    if (prog_idle == NULL)
    {
        printf("[scheduler.c] (process_init_idle) : Can't open prog_idle\n");
        return;
    }

    // Put Base = 0 for idle process
    int base_idle = 0;
    shell_load_prog(prog_idle, base_idle);
    fclose(prog_idle);

    proc_idle = process_init_PCB("prog_idle", base_idle);
}

void process_init(unsigned int tq)
{
    time_quantum = tq;
    process_init_PCBs();
    process_init_readyQ();
    process_init_idle();
}

void process_submit(unsigned int p_base, char *p_fname)
{
    PCB *new_proc = process_init_PCB(p_fname, p_base);
    if (new_proc == NULL)
    {
        printf("[scheduler.c] (process_submit) : New Process (fname = %s) submit FAILED.\n", p_fname);
        return;
    }
    print_init_spool(new_proc->pid);
    process_insert_readyQ(new_proc);
    printf("[scheduler.c] (process_submit) : New Process (PID = %d) submitted.\n", new_proc->pid);
}

void process_exit(PCB *proc, bool pexit)
{
    int exit_pid = proc->pid;
    if (pexit)
    {
        print_end_spool(exit_pid);
    }
    process_dispose_PCB(proc->pid);
    pop_readyQ(rq);
    printf("[scheduler.c] (process_exit) : Process (PID = %d) exited.\n", exit_pid);
}

void scheduler_terminate()
{
    while (rq->size > 0)
    {
        PCB *proc = process_fetch_readyQ();
        process_exit(proc, false);
    }
    printf("[scheduler.c] (scheduler_terminate) : Scheduler shut down complete.\n");

    print_terminate();
}

void process_execute()
{
    bool idle = false;
    while (!shut_down)
    {
        PCB *proc = process_fetch_readyQ();
        if (proc == NULL)
        {
            idle = true;
            // Overwriting reg_state with idle.
            // Since nothing is in the ReadyQ (process -> idle) or (idle -> idle)
            process_context_switch(proc_idle, NULL);
            curr_pid = proc_idle->pid;
        }
        else
        {
            idle = false;
            // printf("[scheduler.c] (process_execute) : Running Process: %d\n", proc->pid);
            
            // Overwriting reg_state with readyQ front.
            // Proper 2 process context switch wouldnt work
            // Consider the case when first process is submitted (idle -> process)
            process_context_switch(proc, NULL);
            curr_pid = proc->pid;
        }

        int proc_state = cpu_operation();
        // -1 = Time Quantum Expired
        // 1 = Process Exit (either intentional or error)
        if (proc_state == -1)
        {
            if (!idle) // (idle -> idle)
            {
                // Context Switch between current and incoming process (process -> process)
                PCB *in = (rotate_readyQ(rq))->proc;
                process_context_switch(in, proc);
                printf("[scheduler.c] (process_execute) : Switching Process.\n");
                printf("    PID in: %d, out: %d\n", in->pid, proc->pid);
                printf("    PC in: %d, out: %d\n\n", in->reg_state.PC.reg_val, proc->reg_state.PC.reg_val);
            }
        }
        else if (proc_state == 1)
        {

            process_exit(proc, true);
        }
        else
        {
            printf("[scheduler.c] (process_execute) : Unexpected CPU status: %d. Shutting Down Now.\n", proc_state);
            break;
        }

        // usleep(1000000);
    }

    printf("[scheduler.c] (process_execute) : Scheduler shut down started.\n");
    scheduler_terminate();
}