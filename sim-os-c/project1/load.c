#include <stdio.h>
#include <stdlib.h>
#include "computer.h"

extern struct Memory MEM;

FILE *load_prog(char *fname, int p_addr)
{
    FILE *prog_f = fopen(fname, "r");
    if (prog_f == NULL)
    {
        printf("[load.c] (load_prog) : Can't open %s\n", fname);
        return NULL;
    }

    int n_code, n_data;
    fscanf(prog_f, "%d %d\n", &n_code, &n_data);

    int i, op_code, operand;
    for (i = p_addr; i < p_addr + 2 * n_code; i += 2)
    {
        fscanf(prog_f, "%d %d\n", &op_code, &operand);
        MEM.mem_arr[i] = op_code;
        MEM.mem_arr[i + 1] = operand;
    }

    for (i = p_addr + 2 * n_code; i < p_addr + 2 * n_code + n_data; ++i)
    {
        fscanf(prog_f, "%d\n", &operand);
        MEM.mem_arr[i] = operand;
    }

    return prog_f;
}

void load_finish(FILE *f)
{
    fclose(f);

    // printf("[load.c] (load_finish) : Clean up complete.\n");
}