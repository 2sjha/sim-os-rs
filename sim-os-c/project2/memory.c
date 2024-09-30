#include <stdlib.h>
#include "computer.h"

Memory mem = {0, NULL};
extern Register_File regs;

void mem_init(unsigned int M)
{
    mem.size = M;
    mem.mem_arr = calloc(M, sizeof(unsigned int));
    for (int i = 0; i < M; ++i)
    {
        mem.mem_arr[i] = 0;
    }
}

void mem_read()
{
    regs.MBR.reg_val = mem.mem_arr[regs.MAR.reg_val];
}

void mem_write()
{
    mem.mem_arr[regs.MAR.reg_val] = regs.MBR.reg_val;
}

void mem_dump()
{
    printf("===========================================\n");
    printf("           Memory Dump: Size = %d\n", mem.size);
    printf("===========================================\n");
    printf("Address: Contents\n");
    for (int i = 0; i < mem.size; ++i)
    {
        printf("%d: %d\n", i, mem.mem_arr[i]);
    }
    printf("\n");
}