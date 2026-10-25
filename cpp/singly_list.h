#ifndef SINGLY_LIST_H
#define SINGLY_LIST_H

#include "common.h"

struct SNode {
    string value;
    SNode* next;
};

struct SinglyList {
    string name;
    SNode* head;
    SNode* tail;
    int size;
};

void singlyInit(SinglyList& l, const string& name);
void singlyClear(SinglyList& l);

CommandResult singlyPush(SinglyList& l, const string& value, const string& mode, const string& target = ""); // target нужен только для after и before
CommandResult singlyDel(SinglyList& l, const string& arg, const string& mode);
CommandResult singlyGet(const SinglyList& l);
CommandResult singlyFind(const SinglyList& l, const string& value);
CommandResult singlyPrint(const SinglyList& l);

#endif