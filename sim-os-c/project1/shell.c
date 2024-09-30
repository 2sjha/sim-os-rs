#include <stdio.h>
#include "computer.h"

extern struct RegisterFile REGS;

extern struct Memory MEM;

void shell_init()
{
    // printf("[shell.c] (shell_init) : Shell init complete.\n");
}

void shell_print_registers()
{
    printf("===========================================\n");
    printf("               Register Dump\n");
    printf("===========================================\n");
    printf("Register: Contents\n");

    printf("BASE: %d\n", REGS.BASE.reg_val);
    printf("PC: %d\n", REGS.PC.reg_val);
    printf("IR0: %d\n", REGS.IR0.reg_val);
    printf("IR1: %d\n", REGS.IR1.reg_val);
    printf("AC: %d\n", REGS.AC.reg_val);
    printf("MAR: %d\n", REGS.MAR.reg_val);
    printf("MBR: %d\n", REGS.MBR.reg_val);
    printf("\n");
}

void shell_print_memory()
{
    printf("===========================================\n");
    printf("           Memory Dump: Size = %d\n", MEM.size);
    printf("===========================================\n");
    printf("Address: Contents\n");
    for (int i = 0; i < MEM.size; ++i)
    {
        printf("%d: %d\n", i, MEM.mem_arr[i]);
    }
    printf("\n");
}

void shell_command(int cmd)
{
    if (cmd == 2)
    {
        shell_print_registers();
    }
    else if (cmd == 3)
    {
        shell_print_memory();
    }
    else
    {
        printf("[shell.c] (shell_command) : Invalid Cmd %d.\n", cmd);
    }
}