#include "dispatcher.h"

// Проверка имени контейнера
static bool checkName(const string& expected, const string& actual) {
    return !expected.empty() && expected == actual;
}

CommandResult executeCommand(Containers& c, const ParsedCommand& pc) {
    const vector<string>& t = pc.tokens;
    if (t.empty()) return makeError("ERROR: empty command");

    const string& cmd = t[0];
	 
	// Универсальная команда PRINT
    if (cmd == "PRINT") {
        if (t.size() < 2) return makeError("ERROR: PRINT requires name");
        const string& name = t[1];
        if (name == c.arr.name) return arrayPrint(c.arr);
        if (name == c.slist.name) return singlyPrint(c.slist);
        if (name == c.dlist.name) return doublyPrint(c.dlist);
        if (name == c.stk.name) return stackPrint(c.stk);
        if (name == c.q.name) return queuePrint(c.q);
        if (name == c.tree.name) return treePrint(c.tree);
        return makeError("ERROR: unknown container name");
    }

	// "Сортировка"
    switch (pc.type) {
        case CT_ARRAY: {
            if (t.size() < 2) return makeError("ERROR: missing name");
            const string& name = t[1];
            if (!checkName(c.arr.name, name)) return makeError("ERROR: name mismatch");
            if (cmd == "MPUSH") {
                if (t.size() == 3) return arrayPush(c.arr, t[2]);				// Без индекса
                if (t.size() == 4) return arrayPush(c.arr, t[2], stoi(t[3]));	// С индексом
                return makeError("ERROR: bad MPUSH args");
            }
            if (cmd == "MDEL") {
                if (t.size() != 3) return makeError("ERROR: bad MDEL args");
                return arrayDel(c.arr, stoi(t[2]));
            }
            if (cmd == "MGET") {
                if (t.size() != 3) return makeError("ERROR: bad MGET args");
                return arrayGet(c.arr, stoi(t[2]));
            }
            if (cmd == "MSET") {
                if (t.size() != 4) return makeError("ERROR: bad MSET args");
                return arraySet(c.arr, stoi(t[2]), t[3]);
            }
            if (cmd == "MLEN") {
                return arrayLen(c.arr);
            }
            return makeError("ERROR: unknown array command");
        }
        case CT_SINGLY: {
            if (t.size() < 2) return makeError("ERROR: missing name");
            const string& name = t[1];
            if (!checkName(c.slist.name, name)) return makeError("ERROR: name mismatch");
            if (cmd == "FPUSH") {
                if (t.size() == 4) return singlyPush(c.slist, t[2], t[3]);
                if (t.size() == 5) return singlyPush(c.slist, t[2], t[3], t[4]);
                return makeError("ERROR: bad FPUSH args");
            }
            if (cmd == "FDEL") {
                if (t.size() == 3) return singlyDel(c.slist, "", t[2]);
                if (t.size() == 4) return singlyDel(c.slist, t[2], t[3]);
                return makeError("ERROR: bad FDEL args");
            }
            if (cmd == "FGET") return singlyGet(c.slist);
            if (cmd == "FFIND") {
                if (t.size() != 3) return makeError("ERROR: bad FFIND args");
                return singlyFind(c.slist, t[2]);
            }
            return makeError("ERROR: unknown singly command");
        }
        case CT_DOUBLY: {
            if (t.size() < 2) return makeError("ERROR: missing name");
            const string& name = t[1];
            if (!checkName(c.dlist.name, name)) return makeError("ERROR: name mismatch");
            if (cmd == "LPUSH") {
                if (t.size() == 4) return doublyPush(c.dlist, t[2], t[3]);
                if (t.size() == 5) return doublyPush(c.dlist, t[2], t[3], t[4]);
                return makeError("ERROR: bad LPUSH args");
            }
            if (cmd == "LDEL") {
                if (t.size() == 3) return doublyDel(c.dlist, "", t[2]);
                if (t.size() == 4) return doublyDel(c.dlist, t[2], t[3]);
                return makeError("ERROR: bad LDEL args");
            }
            if (cmd == "LGET") return doublyGet(c.dlist);
            if (cmd == "LFIND") {
                if (t.size() != 3) return makeError("ERROR: bad LFIND args");
                return doublyFind(c.dlist, t[2]);
            }
            return makeError("ERROR: unknown doubly command");
        }
        case CT_STACK: {
            if (t.size() < 2) return makeError("ERROR: missing name");
            const string& name = t[1];
            if (!checkName(c.stk.name, name)) return makeError("ERROR: name mismatch");
            if (cmd == "SPUSH") {
                if (t.size() != 3) return makeError("ERROR: bad SPUSH args");
                return stackPush(c.stk, t[2]);
            }
            if (cmd == "SPOP") return stackPop(c.stk);
            return makeError("ERROR: unknown stack command");
        }
        case CT_QUEUE: {
            if (t.size() < 2) return makeError("ERROR: missing name");
            const string& name = t[1];
            if (!checkName(c.q.name, name)) return makeError("ERROR: name mismatch");
            if (cmd == "QPUSH") {
                if (t.size() != 3) return makeError("ERROR: bad QPUSH args");
                return queuePush(c.q, t[2]);
            }
            if (cmd == "QPOP") return queuePop(c.q);
            return makeError("ERROR: unknown queue command");
        }
        case CT_TREE: {
            if (t.size() < 2) return makeError("ERROR: missing name");
            const string& name = t[1];
            if (!checkName(c.tree.name, name)) return makeError("ERROR: name mismatch");
            if (cmd == "TINSERT") {
                if (t.size() != 3) return makeError("ERROR: bad TINSERT args");
                return treeInsert(c.tree, stoi(t[2]));
            }
            if (cmd == "TGET") {
                if (t.size() != 3) return makeError("ERROR: bad TGET args");
                return treeGet(c.tree, stoi(t[2]));
            }
            if (cmd == "TCHECK") return treeCheck(c.tree);
            return makeError("ERROR: unknown tree command");
        }
        default:
            return makeError("ERROR: unknown command");
    }
}