#include "computer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern struct RegisterFile REGS;

struct PCB Process;

int parse_int(char *num)
{
    int size = strlen(num);
    int sum = 0;
    for (int i = 0; i < size; ++i)
    {
        sum = 10 * sum + ((int)num[i] - 48);
    }
    return sum;
}

int process_init_PCB(int base)
{
    REGS.BASE.reg_val = base;

    Process.PID = 1;
    Process.REG_STATE = REGS;

    return Process.PID;
}

void process_set_registers()
{
    REGS.BASE.reg_val = 0;
    REGS.PC.reg_val = 0;
    REGS.IR0.reg_val = -1;
    REGS.IR1.reg_val = -1;
    REGS.AC.reg_val = 0;
    REGS.MAR.reg_val = 0;
    REGS.MBR.reg_val = 0;
}

void boot_system(int mem_size)
{
    mem_init(mem_size);
    shell_init();
    process_set_registers();
}

int main(int argc, char **argv)
{
    char *config_fname = "config.sys";
    FILE *config_fp = fopen(config_fname, "r");
    if (config_fp == NULL)
    {
        printf("[computer.c] (main) : Can't open %s\n", config_fname);
        return 1;
    }

    unsigned int mem_size;
    fscanf(config_fp, "%d", &mem_size);

    boot_system(mem_size);

    char *prog_fname = malloc(20 + 1);
    int base;
    if (argc == 1)
    {
        printf("Input Program File and Base Address: ");
        scanf("%s %d", prog_fname, &base);
    }
    else if (argc == 3)
    {
        prog_fname = argv[1];
        base = parse_int(argv[2]);
    }
    else
    {
        printf("[computer.c] (main) : Invalid Usage. Valid uses are \n./computer.exe\n./computer.exe comp.in 8\n");
    }

    FILE *prog_fp = load_prog(prog_fname, base);
    if (prog_fp == NULL)
    {
        return 1;
    }

    int PID = process_init_PCB(base);

    cpu_operation();

    load_finish(prog_fp);
    return 0;
}