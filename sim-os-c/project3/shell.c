#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <pthread.h>
#include <unistd.h>
#include "computer.h"

bool shut_down;
extern Memory mem;

void shell_load_prog(FILE *prog_f, int p_addr)
{
    int n_code, n_data;
    fscanf(prog_f, "%d %d\n", &n_code, &n_data);

    int i, op_code, operand;
    for (i = p_addr; i < p_addr + 2 * n_code; i += 2)
    {
        fscanf(prog_f, "%d %d\n", &op_code, &operand);
        mem.mem_arr[i] = op_code;
        mem.mem_arr[i + 1] = operand;
    }

    for (i = p_addr + 2 * n_code; i < p_addr + 2 * n_code + n_data; ++i)
    {
        fscanf(prog_f, "%d\n", &operand);
        mem.mem_arr[i] = operand;
    }
}

void shell_terminate_system()
{
    fprintf(stdout, "[shell.c] (shell_terminate_system) : Shell shut down started.\n");
    shut_down = true;
}

void shell_process_submit()
{
    char prog_fname[33];
    int base = 0;

    fprintf(stdout, "Input Program File and Base Address: \n");
    scanf("%s %d", prog_fname, &base);

    FILE *prog_f = fopen(prog_fname, "r");
    if (prog_f == NULL)
    {
        fprintf(stderr, "[shell.c] (shell_process_submit) : Can't open %s\n", prog_fname);
        return;
    }
    shell_load_prog(prog_f, base);
    fclose(prog_f);

    process_submit(base, prog_fname);
}

void shell_print_registers()
{
    cpu_reg_dump();
}

void shell_print_memory()
{
    mem_dump();
}

void shell_print_readyQ()
{
    process_dump_readyQ();
}

void shell_print_PCB()
{
    process_dump_PCBs();
}

void shell_print_spools()
{
    print_spool_dump();
}

void shell_print_all_spools()
{
    printer_manager_all_spools_dump();
}

void shell_instruction(int cmd)
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
        fprintf(stderr, "[shell.c] (shell_instruction) : Invalid Cmd %d.\n", cmd);
    }
}

void printer_shell_command(int cmd)
{
    if (cmd == 0)
    {
        shell_terminate_system();
        printer_manager_terminate();
    }
    else if (cmd == 6)
    {
        shell_print_all_spools();
    }
    else
    {
        fprintf(stderr, "[shell.c] (printer_shell_command) : Invalid Cmd %d.\n", cmd);
    }
}

void shell_command(int cmd)
{
    if (cmd == 0)
    {
        shell_terminate_system();
    }
    else if (cmd == 1)
    {
        shell_process_submit();
    }
    else if (cmd == 2)
    {
        shell_print_registers();
    }
    else if (cmd == 3)
    {
        shell_print_memory();
    }
    else if (cmd == 4)
    {
        shell_print_readyQ();
    }
    else if (cmd == 5)
    {
        shell_print_PCB();
    }
    else if (cmd == 6)
    {
        shell_print_spools();
    }
    else
    {
        fprintf(stderr, "[shell.c] (shell_command) : Invalid Command %d.\n", cmd);
    }
}

void *shell_operation(void *arg)
{
    int cmd = -1;
    while (!shut_down)
    {
        fprintf(stdout, "Input Shell Command: \n");
        scanf("%d", &cmd);
        shell_command(cmd);
        // usleep(1000000);
    }

    fprintf(stdout, "[shell.c] (shell_operation) : Shell shut down complete.\n");
}

void *printer_shell_operation(void *arg)
{
    int cmd = -1;
    while (!shut_down)
    {
        fprintf(stdout, "Input Shell Command: \n");
        fprintf(stdout, "0: Printer Shutdown & 6: All Spool Dump\n");
        scanf("%d", &cmd);
        printer_shell_command(cmd);
    }

    fprintf(stdout, "[shell.c] (printer_shell_operation) : Printer Shell shut down complete.\n");
}