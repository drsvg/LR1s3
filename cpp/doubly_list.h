#ifndef DOUBLY_LIST_H
#define DOUBLY_LIST_H

#include "common.h"

struct DNode {
    string value;
    DNode* prev;
    DNode* next;
};

struct DoublyList {
    string name;
    DNode* head;
    DNode* tail;
    int size;
};

void doublyInit(DoublyList& l, const string& name);
void doublyClear(DoublyList& l);

CommandResult doublyPush(DoublyList& l, const string& value, const string& mode, const string& target = "");
CommandResult doublyDel(DoublyList& l, const string& arg, const string& mode);
CommandResult doublyGet(const DoublyList& l);
CommandResult doublyFind(const DoublyList& l, const string& value);
CommandResult doublyPrint(const DoublyList& l);

#endif