#define _GNU_SOURCE
#include <stdio.h>
#include <unistd.h>
#include "computer.h"

struct RegisterFile REGS = {{0}, {0}, {0}, {0}, {0}, {0}, {0}};

int cpu_mem_address(int m_addr)
{
    REGS.MAR.reg_val = REGS.BASE.reg_val + m_addr;
    mem_read();
    return REGS.MBR.reg_val;
}

void cpu_fetch_instruction()
{
    REGS.IR0.reg_val = cpu_mem_address(REGS.PC.reg_val);
    REGS.IR1.reg_val = cpu_mem_address(REGS.PC.reg_val + 1);
    REGS.PC.reg_val += 2;
}

void cpu_execute_instruction()
{
    if (REGS.IR0.reg_val == 1)
    {
        REGS.AC.reg_val = REGS.IR1.reg_val;
    }
    else if (REGS.IR0.reg_val == 2)
    {
        REGS.AC.reg_val = cpu_mem_address(REGS.IR1.reg_val);
    }
    else if (REGS.IR0.reg_val == 3)
    {
        cpu_mem_address(REGS.IR1.reg_val);
        REGS.AC.reg_val += REGS.MBR.reg_val;
    }
    else if (REGS.IR0.reg_val == 4)
    {
        cpu_mem_address(REGS.IR1.reg_val);
        REGS.AC.reg_val *= REGS.MBR.reg_val;
    }
    else if (REGS.IR0.reg_val == 5)
    {
        REGS.MBR.reg_val = REGS.AC.reg_val;
        REGS.MAR.reg_val = REGS.BASE.reg_val + REGS.IR1.reg_val;
        mem_write();
    }
    else if (REGS.IR0.reg_val == 6)
    {
        if (REGS.AC.reg_val > 0)
        {
            REGS.PC.reg_val = REGS.IR1.reg_val;
        }
    }
    else if (REGS.IR0.reg_val == 7)
    {
        printf("%d\n", REGS.AC.reg_val);
    }
    else if (REGS.IR0.reg_val == 8)
    {
        usleep(REGS.IR1.reg_val);
    }
    else if (REGS.IR0.reg_val == 9)
    {
        shell_command(REGS.IR1.reg_val);
    }
    else if (REGS.IR0.reg_val == 0)
    {
        return;
    }
    else
    {
        printf("[cpu.c] (cpu_execute_instruction) : Invalid Instruction. Exiting.\n");
        REGS.IR0.reg_val = 0;
        return;
    }
}

void cpu_operation()
{
    while (REGS.IR0.reg_val != 0)
    {
        cpu_fetch_instruction();
        cpu_execute_instruction();
    }
}