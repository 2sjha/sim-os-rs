#include <string.h>
#include <stdlib.h>
#include <ctype.h>
#include "computer.h"

void init_readyQ(ReadyQ *rq)
{
    if (rq == NULL)
    {
        return;
    }

    rq->size = 0;
    rq->head = calloc(1, sizeof(ReadyQ_Node));
    rq->tail = calloc(1, sizeof(ReadyQ_Node));
    rq->head->proc = NULL;
    rq->head->prev = NULL;
    rq->head->next = rq->tail;
    rq->tail->proc = NULL;
    rq->tail->prev = rq->head;
    rq->tail->next = NULL;
}

void push_readyQ(ReadyQ *rq, PCB *proc)
{
    ReadyQ_Node *rqn = calloc(1, sizeof(ReadyQ_Node));
    rqn->proc = proc;
    rqn->prev = rq->tail->prev;
    rqn->next = rq->tail;
    rq->tail->prev->next = rqn;
    rq->tail->prev = rqn;
    rq->size++;
}

void pop_readyQ(ReadyQ *rq)
{
    if (rq == NULL || rq->size == 0)
    {
        return;
    }

    ReadyQ_Node *rqn = rq->head->next;
    rq->head->next = rqn->next;
    rqn->next->prev = rq->head;
    rq->size--;
    free(rqn);
}

ReadyQ_Node *rotate_readyQ(ReadyQ *rq)
{
    if (rq == NULL || rq->size == 0)
    {
        return NULL;
    }

    ReadyQ_Node *front = rq->head->next;
    rq->head->next = front->next;
    front->next->prev = rq->head;

    front->prev = rq->tail->prev;
    front->next = rq->tail;
    rq->tail->prev->next = front;
    rq->tail->prev = front;

    return rq->head->next;
}

void dump_readyQ(ReadyQ *rq)
{
    if (rq->size == 0)
    {
        fprintf(stdout, "0: 1 (Idle)\n");
        return;
    }

    int index = 0;
    ReadyQ_Node *curr = rq->head->next;
    fprintf(stdout, "%d: %d (Running)\n", index, curr->proc->pid);
    curr = curr->next;
    index++;

    while (curr != rq->tail)
    {
        fprintf(stdout, "%d: %d\n", index, curr->proc->pid);
        curr = curr->next;
        index++;
    }
}

unsigned int parse_int(char *num)
{
    unsigned int size = strlen(num);
    unsigned int sum = 0;
    for (int i = 0; i < size; ++i)
    {
        if (!isdigit(num[i]))
        {
            return -1;
        }

        int digit = (int)num[i] - 48;
        sum = 10 * sum + digit;
    }
    return sum;
}