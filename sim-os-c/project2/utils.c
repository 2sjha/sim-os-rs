#include <string.h>
#include <stdlib.h>
#include "computer.h"

void init_readyQ(ReadyQ *rq)
{
    if (rq == NULL)
    {
        return;
    }

    rq->size = 0;
    rq->head = malloc(sizeof(ReadyQ_Node));
    rq->tail = malloc(sizeof(ReadyQ_Node));
    rq->head->proc = NULL;
    rq->head->prev = NULL;
    rq->head->next = rq->tail;
    rq->tail->proc = NULL;
    rq->tail->prev = rq->head;
    rq->tail->next = NULL;
}

void push_readyQ(ReadyQ *rq, PCB *proc)
{
    ReadyQ_Node *rqn = malloc(sizeof(ReadyQ_Node));
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
        printf("0: 1 (Idle)\n");
        return;
    }

    int index = 0;
    ReadyQ_Node *curr = rq->head->next;
    printf("%d: %d (Running)\n", index, curr->proc->pid);
    curr = curr->next;
    index++;

    while (curr != rq->tail)
    {
        printf("%d: %d\n", index, curr->proc->pid);
        curr = curr->next;
        index++;
    }
}