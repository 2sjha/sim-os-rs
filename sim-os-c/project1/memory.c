#include <stdlib.h>
#include "computer.h"

extern struct RegisterFile REGS;

struct Memory MEM = {0, NULL};

void mem_init(int M)
{
    MEM.size = M;
    MEM.mem_arr = malloc(M * sizeof(int));
    for (int i = 0; i < M; ++i)
    {
        *(MEM.mem_arr + i) = 0;
    }
}

void mem_read()
{
    REGS.MBR.reg_val = MEM.mem_arr[REGS.MAR.reg_val];
}

void mem_write()
{
    MEM.mem_arr[REGS.MAR.reg_val] = REGS.MBR.reg_val;
}