#ifndef CBTREE_H
#define CBTREE_H

#include "common.h"

struct TNode {
    int value;
    TNode* left;
    TNode* right;
};

struct CBTree {
    string name;
    TNode* root; // корень - самый верхний элемент
    int size;
};

void treeInit(CBTree& t, const string& name);
void treeClear(CBTree& t);
CommandResult treeInsert(CBTree& t, int value);
CommandResult treeGet(const CBTree& t, int value);   // поиск
CommandResult treeCheck(const CBTree& t);            // проверка на Complete
CommandResult treePrint(const CBTree& t);

#endif