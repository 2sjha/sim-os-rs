#define _GNU_SOURCE
#include <stdio.h>
#include <stdbool.h>
#include <unistd.h>
#include <string.h>
#include "computer.h"

Register_File regs = {{0}, {0}, {0}, {0}, {0}, {0}, {0}};
extern unsigned int curr_pid;
extern unsigned int time_quantum;
char cpu_print_buf[33];

int cpu_mem_address(int m_addr)
{
    regs.MAR.reg_val = regs.BASE.reg_val + m_addr;
    mem_read();
    return regs.MBR.reg_val;
}

void cpu_fetch_instruction()
{
    regs.IR0.reg_val = cpu_mem_address(regs.PC.reg_val);
    regs.IR1.reg_val = cpu_mem_address(regs.PC.reg_val + 1);
    regs.PC.reg_val += 2;
}

void cpu_execute_instruction()
{
    if (regs.IR0.reg_val == 1)
    {
        regs.AC.reg_val = regs.IR1.reg_val;
    }
    else if (regs.IR0.reg_val == 2)
    {
        regs.AC.reg_val = cpu_mem_address(regs.IR1.reg_val);
    }
    else if (regs.IR0.reg_val == 3)
    {
        cpu_mem_address(regs.IR1.reg_val);
        regs.AC.reg_val += regs.MBR.reg_val;
    }
    else if (regs.IR0.reg_val == 4)
    {
        cpu_mem_address(regs.IR1.reg_val);
        regs.AC.reg_val *= regs.MBR.reg_val;
    }
    else if (regs.IR0.reg_val == 5)
    {
        regs.MBR.reg_val = regs.AC.reg_val;
        regs.MAR.reg_val = regs.BASE.reg_val + regs.IR1.reg_val;
        mem_write();
    }
    else if (regs.IR0.reg_val == 6)
    {
        if (regs.AC.reg_val > 0)
        {
            regs.PC.reg_val = regs.IR1.reg_val;
        }
    }
    else if (regs.IR0.reg_val == 7)
    {
        memset(cpu_print_buf, 0, sizeof(cpu_print_buf));
        snprintf(cpu_print_buf, sizeof(cpu_print_buf), "AC:%d", regs.AC.reg_val);
        print_print(curr_pid, cpu_print_buf);
    }
    else if (regs.IR0.reg_val == 8)
    {
        usleep(regs.IR1.reg_val);
    }
    else if (regs.IR0.reg_val == 9)
    {
        shell_instruction(regs.IR1.reg_val);
    }
    else if (regs.IR0.reg_val == 0)
    {
        return;
    }
    else
    {
        fprintf(stderr, "[cpu.c] (cpu_execute_instruction) : Invalid Instruction: %d. Exiting.\n", regs.IR0.reg_val);
        regs.IR0.reg_val = 0;
        return;
    }
}

int cpu_operation()
{
    for (int i = 0; i < time_quantum; ++i)
    {
        if (regs.IR0.reg_val == 0)
        {
            return 1;
        }

        cpu_fetch_instruction();
        cpu_execute_instruction();
    }

    return -1;
}

void cpu_reg_dump()
{
    fprintf(stdout, "===========================================\n");
    fprintf(stdout, "               Register Dump\n");
    fprintf(stdout, "===========================================\n");
    fprintf(stdout, "Register: Contents\n");

    fprintf(stdout, "BASE: %d\n", regs.BASE.reg_val);
    fprintf(stdout, "PC: %d\n", regs.PC.reg_val);
    fprintf(stdout, "IR0: %d\n", regs.IR0.reg_val);
    fprintf(stdout, "IR1: %d\n", regs.IR1.reg_val);
    fprintf(stdout, "AC: %d\n", regs.AC.reg_val);
    fprintf(stdout, "MAR: %d\n", regs.MAR.reg_val);
    fprintf(stdout, "MBR: %d\n", regs.MBR.reg_val);
    fprintf(stdout, "\n");
}