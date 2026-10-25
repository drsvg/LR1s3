#include "cbtree.h"
#include <queue>

void treeInit(CBTree& t, const string& name) {
    t.name = name;
    t.root = nullptr;
    t.size = 0;
}

static void clearNode(TNode* n) {
    if (!n) return;
    clearNode(n->left);
    clearNode(n->right);
    delete n;
}

void treeClear(CBTree& t) {
    clearNode(t.root);
    t.root = nullptr;
    t.size = 0;
}

// Вставка в первую свободную позицию по BFS (обход в ширину)
CommandResult treeInsert(CBTree& t, int value) {
    TNode* node = new TNode;
    node->value = value;
    node->left = nullptr;
    node->right = nullptr;

	// Если дерево пустое, новый узел становится корнем
    if (!t.root) {
        t.root = node;
        t.size++;
        return makeOk(to_string(value));
    }

    // BFS: ищем первый узел, у которого нет левого или правого ребенка
    queue<TNode*> q;
    q.push(t.root);
    while (!q.empty()) {
        TNode* cur = q.front();
        q.pop();
        if (!cur->left) {
            cur->left = node;
            t.size++;
            return makeOk(to_string(value));
        }
        if (!cur->right) {
            cur->right = node;
            t.size++;
            return makeOk(to_string(value));
        }
        q.push(cur->left);
        q.push(cur->right);
    }
    delete node;
    return makeError("ERROR: insertion failed");
}

static bool findNode(TNode* n, int value) {
    if (!n) return false;
    if (n->value == value) return true;
    return findNode(n->left, value) || findNode(n->right, value);
}

CommandResult treeGet(const CBTree& t, int value) {
    return makeOk(findNode(t.root, value) ? "TRUE" : "FALSE");
}

// Проверка на Complete Binary Tree
static bool isComplete(TNode* root, int index, int total) {
    if (!root) return true;
    if (index >= total) return false;
    return isComplete(root->left, 2 * index + 1, total) &&
           isComplete(root->right, 2 * index + 2, total);
}

CommandResult treeCheck(const CBTree& t) {
    bool ok = isComplete(t.root, 0, t.size);
    return makeOk(ok ? "TRUE" : "FALSE");
}

static void printNode(TNode* n, ostringstream& oss) {
    if (!n) return;
    oss << n->value << " ";
    printNode(n->left, oss);
    printNode(n->right, oss);
}

CommandResult treePrint(const CBTree& t) {
    ostringstream oss;
    printNode(t.root, oss);
    string s = oss.str();
    if (!s.empty() && s.back() == ' ') s.pop_back();
    return makeOk("[" + s + "]");
}