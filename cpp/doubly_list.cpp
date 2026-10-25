#include "doubly_list.h"

void doublyInit(DoublyList& l, const string& name) {
    l.name = name;
    l.head = nullptr;
    l.tail = nullptr;
    l.size = 0;
}

void doublyClear(DoublyList& l) {
    DNode* cur = l.head;
    while (cur) {
        DNode* nxt = cur->next;
        delete cur;
        cur = nxt;
    }
    l.head = nullptr;
    l.tail = nullptr;
    l.size = 0;
}

CommandResult doublyPush(DoublyList& l, const string& value, const string& mode, const string& target) {
    DNode* node = new DNode;
    node->value = value;
    node->prev = nullptr;
    node->next = nullptr;

    if (mode == "head") {
        node->next = l.head;
        if (l.head) l.head->prev = node;
        l.head = node;
        if (!l.tail) l.tail = node;
    } else if (mode == "tail") {
        node->prev = l.tail;
        if (l.tail) l.tail->next = node;
        l.tail = node;
        if (!l.head) l.head = node;
    } else if (mode == "after") {
        if (target.empty()) {
            delete node;
            return makeError("ERROR: target required for 'after' mode");
        }
        DNode* cur = l.head;
        while (cur && cur->value != target) cur = cur->next;
        if (!cur) {
            delete node;
            return makeError("ERROR: target not found");
        }
        node->prev = cur;
        node->next = cur->next;
        if (cur->next) cur->next->prev = node;
        else l.tail = node;
        cur->next = node;
    } else if (mode == "before") {
        if (target.empty()) {
            delete node;
            return makeError("ERROR: target required for 'before' mode");
        }
        DNode* cur = l.head;
        while (cur && cur->value != target) cur = cur->next;
        if (!cur) {
            delete node;
            return makeError("ERROR: target not found");
        }
        node->next = cur;
        node->prev = cur->prev;
        if (cur->prev) cur->prev->next = node;
        else l.head = node;
        cur->prev = node;
    } else {
        delete node;
        return makeError("ERROR: unknown mode");
    }
    l.size++;
    return makeOk(value);
}

CommandResult doublyDel(DoublyList& l, const string& arg, const string& mode) {
    if (!l.head) return makeError("ERROR: list is empty");

    if (mode == "head") {
        DNode* del = l.head;
        l.head = l.head->next;
        if (l.head) l.head->prev = nullptr;
        else l.tail = nullptr;
        string v = del->value;
        delete del;
        l.size--;
        return makeOk(v);
    }
    if (mode == "tail") {
        DNode* del = l.tail;
        l.tail = l.tail->prev;
        if (l.tail) l.tail->next = nullptr;
        else l.head = nullptr;
        string v = del->value;
        delete del;
        l.size--;
        return makeOk(v);
    }
    if (mode == "value") {
        DNode* cur = l.head;
        while (cur && cur->value != arg) cur = cur->next;
        if (!cur) return makeError("ERROR: value not found");
        string v = cur->value;
        if (cur->prev) cur->prev->next = cur->next;
        else l.head = cur->next;
        if (cur->next) cur->next->prev = cur->prev;
        else l.tail = cur->prev;
        delete cur;
        l.size--;
        return makeOk(v);
    }
    return makeError("ERROR: unknown mode");
}

CommandResult doublyGet(const DoublyList& l) {
    ostringstream oss;
    oss << "[";
    DNode* cur = l.head;
    bool first = true;
    while (cur) {
        if (!first) oss << ", ";
        oss << cur->value;
        first = false;
        cur = cur->next;
    }
    oss << "]";
    return makeOk(oss.str());
}

CommandResult doublyFind(const DoublyList& l, const string& value) {
    DNode* cur = l.head;
    while (cur) {
        if (cur->value == value) return makeOk("TRUE");
        cur = cur->next;
    }
    return makeOk("FALSE");
}

CommandResult doublyPrint(const DoublyList& l) {
    return doublyGet(l);
}