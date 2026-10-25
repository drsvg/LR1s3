#include "singly_list.h"

void singlyInit(SinglyList& l, const string& name) {
    l.name = name;
    l.head = nullptr;
    l.tail = nullptr;
    l.size = 0;
}

void singlyClear(SinglyList& l) {
    SNode* cur = l.head;
    while (cur) {
        SNode* nxt = cur->next;
        delete cur;
        cur = nxt;
    }
    l.head = nullptr;
    l.tail = nullptr;
    l.size = 0;
}

CommandResult singlyPush(SinglyList& l, const string& value, const string& mode, const string& target) {
    SNode* node = new SNode;
    node->value = value;
    node->next = nullptr;
	
    if (mode == "head") {
        node->next = l.head;
        l.head = node;
        if (!l.tail) l.tail = node;
    } else if (mode == "tail") {
        if (l.tail) l.tail->next = node;
        else l.head = node;
        l.tail = node;
    } else if (mode == "after") {
        if (target.empty()) {
            delete node;
            return makeError("ERROR: target required for 'after' mode");
        }
        SNode* cur = l.head;
        while (cur && cur->value != target) cur = cur->next;
        if (!cur) {
            delete node;
            return makeError("ERROR: target not found");
        }
        node->next = cur->next;
        cur->next = node;
        if (cur == l.tail) l.tail = node; // если вставили после хвоста
    } else if (mode == "before") {
        if (target.empty()) {
            delete node;
            return makeError("ERROR: target required for 'before' mode");
        }
        if (l.head && l.head->value == target) {
            node->next = l.head;
            l.head = node;
        } else {
            SNode* cur = l.head;
            while (cur && cur->next && cur->next->value != target) cur = cur->next;
            if (!cur || !cur->next) {
                delete node;
                return makeError("ERROR: target not found");
            }
            node->next = cur->next;
            cur->next = node;
        }
    } else {
        delete node;
        return makeError("ERROR: unknown mode");
    }
    l.size++;
    return makeOk(value);
}

CommandResult singlyDel(SinglyList& l, const string& arg, const string& mode) {
    if (!l.head) return makeError("ERROR: list is empty");

    if (mode == "head") {
        SNode* del = l.head;
        l.head = l.head->next;
        if (!l.head) l.tail = nullptr;
        string v = del->value;
        delete del;
        l.size--;
        return makeOk(v);
    }
    if (mode == "tail") {
        if (l.head == l.tail) {
            string v = l.head->value;
            delete l.head;
            l.head = nullptr;
            l.tail = nullptr;
            l.size--;
            return makeOk(v);
        }
        SNode* cur = l.head;
        while (cur->next != l.tail) cur = cur->next;
        string v = l.tail->value;
        delete l.tail;
        l.tail = cur;
        l.tail->next = nullptr;
        l.size--;
        return makeOk(v);
    }
    if (mode == "value") {
        SNode* cur = l.head;
        SNode* prev = nullptr;
        while (cur && cur->value != arg) {
            prev = cur;
            cur = cur->next;
        }
        if (!cur) return makeError("ERROR: value not found");
        string v = cur->value;
        if (prev) prev->next = cur->next;
        else l.head = cur->next;
        if (cur == l.tail) l.tail = prev;
        delete cur;
        l.size--;
        return makeOk(v);
    }
    return makeError("ERROR: unknown mode");
}

CommandResult singlyGet(const SinglyList& l) {
    ostringstream oss;
    oss << "[";
    SNode* cur = l.head;
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

CommandResult singlyFind(const SinglyList& l, const string& value) {
    SNode* cur = l.head;
    while (cur) {
        if (cur->value == value) return makeOk("TRUE");
        cur = cur->next;
    }
    return makeOk("FALSE");
}

CommandResult singlyPrint(const SinglyList& l) {
    return singlyGet(l);
}