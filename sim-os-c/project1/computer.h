#include <stdio.h>

/* Simulated Register*/
struct Register
{
    int reg_val;
};

/*Simulated RegisterFile*/
struct RegisterFile
{
    struct Register PC;
    struct Register IR0;
    struct Register IR1;
    struct Register AC;
    struct Register MAR;
    struct Register MBR;
    struct Register BASE;
};

/*Simulated Memory*/
struct Memory
{
    int size;
    int *mem_arr;
};

/*PCB*/
struct PCB
{
    struct RegisterFile REG_STATE;
    int PID;
};

/*Memory Functions*/
void mem_init(int M);
void mem_read();
void mem_write();

/*CPU Functions*/
void cpu_operation();

/*Shell Functions*/
void shell_init();
void shell_print_memory();
void shell_print_registers();
void shell_command(int cmd);

/*Loader Functions*/
FILE *load_prog(char *fname, int base);
void load_finish();