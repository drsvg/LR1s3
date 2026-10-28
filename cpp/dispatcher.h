#ifndef DISPATCHER_H
#define DISPATCHER_H

#include "common.h"
#include "command_parser.h"
#include "storage.h"

struct AppState {
    StorageMeta meta;
};

#include "array.h"
#include "singly_list.h"
#include "doubly_list.h"
#include "stack.h"
#include "queue.h"
#include "cbtree.h"

struct Containers {
    Array arr;
    SinglyList slist;
    DoublyList dlist;
    Stack stk;
    Queue q;
    CBTree tree;
};

CommandResult executeCommand(Containers& c, const ParsedCommand& pc);

#endif