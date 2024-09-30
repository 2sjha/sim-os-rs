#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include "computer.h"

extern unsigned int PRINTER_CID;
unsigned int cid = 0;

void boot_system(unsigned int mem_size, unsigned int tq, unsigned int pt, char *pm_ip,
                 unsigned int pm_port, unsigned int n_comms, unsigned int conn_qsize, unsigned int msg_qsize)
{
    // Inititiate Memory and Scheduler on client computers only
    if (cid != PRINTER_CID)
    {
        mem_init(mem_size);
        process_init(tq);
        print_init(pm_ip, pm_port);
    }
    else
    {
        printer_manager_init(pm_ip, pm_port, n_comms, conn_qsize, msg_qsize);
        printer_init(pt, n_comms);
    }
}

int validate_cid(int read_cid)
{
    if (read_cid < 0)
    {
        fprintf(stderr, "Usage : ./computer.exe <CID>.\n");
        fprintf(stderr, "CID=%d revered for Printer, Other integer CIDs act as client computers.\n", PRINTER_CID);
        exit(1);
    }
    else if (read_cid == PRINTER_CID)
    {
        fprintf(stdout, "[computer.c] (main) : Computer (CID: %d) acting as Printer.\n", PRINTER_CID);
    }
    return read_cid;
}

int main(int argc, char **argv)
{
    if (argc == 2)
    {
        int read_cid = parse_int(argv[1]);
        cid = validate_cid(read_cid);
    }
    else
    {
        fprintf(stderr, "Usage : ./computer.exe <CID>\n");
        fprintf(stderr, "CID=%d revered for Printer, Other integer CIDs act as client computers.\n", PRINTER_CID);
        fprintf(stdout, "Input CID: ");
        scanf("%u", &cid);
        fprintf(stdout, "\n");

        cid = validate_cid(cid);
    }

    char *config_fname = "config.sys";
    FILE *config_fp = fopen(config_fname, "r");
    if (config_fp == NULL)
    {
        fprintf(stderr, "[computer.c] (main) : Can't open file %s\n", config_fname);
        return 1;
    }

    char pm_ip[16];
    unsigned int pm_port, mem_size, time_quantum, print_time, communicators, conn_qsize, msg_qsize;
    fscanf(config_fp, "PM_IP:%s\nPM_PORT:%d\nM:%d\nTQ:%d\nPT:%d\nNC:%d\nCQS:%d\nMQS:%d", pm_ip, &pm_port, &mem_size, &time_quantum, &print_time, &communicators, &conn_qsize, &msg_qsize);
    fclose(config_fp);

    boot_system(mem_size, time_quantum, print_time, pm_ip, pm_port, communicators, conn_qsize, msg_qsize);

    // Run shell and scheduler on client computers only
    if (cid != PRINTER_CID)
    {
        pthread_t shell_tid;
        pthread_create(&shell_tid, NULL, shell_operation, NULL);

        process_execute();
    }
    else
    {
        pthread_t printer_shell_tid;
        pthread_create(&printer_shell_tid, NULL, printer_shell_operation, NULL);

        printer_manager_main();
    }

    return 0;
}