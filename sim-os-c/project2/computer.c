#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include "computer.h"

extern unsigned int child_pid;

void boot_system(unsigned int mem_size, unsigned int time_quantum, unsigned int print_time)
{
    mem_init(mem_size);
    process_init(time_quantum);
    print_init(print_time);
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

    unsigned int mem_size, time_quantum, print_time;
    fscanf(config_fp, "M:%d\nTQ:%d\nPT:%d", &mem_size, &time_quantum, &print_time);
    fclose(config_fp);

    boot_system(mem_size, time_quantum, print_time);

    // Run shell and scheduler on Parent process only
    if (child_pid != 0)
    {
        pthread_t shell_tid;
        pthread_create(&shell_tid, NULL, shell_operation, NULL);

        process_execute();
    }

    return 0;
}