// dispatcher.rs
use crate::common::{CommandResult, ContainerType, make_ok, make_error};
use crate::command_parser::ParsedCommand;
use crate::array::Array;
use crate::singly_list::SinglyList;
use crate::doubly_list::DoublyList;
use crate::stack::Stack;
use crate::queue::Queue;
use crate::cbtree::CBTree;
use crate::storage::StorageMeta;

pub struct Containers {
    pub arr: Array,
    pub slist: SinglyList,
    pub dlist: DoublyList,
    pub stk: Stack,
    pub q: Queue,
    pub tree: CBTree,
}

impl Containers {
    pub fn init(meta: &StorageMeta) -> Self {
        Containers {
            arr: Array::init(&meta.array_name),
            slist: SinglyList::init(&meta.singly_name),
            dlist: DoublyList::init(&meta.doubly_name),
            stk: Stack::init(&meta.stack_name),
            q: Queue::init(&meta.queue_name),
            tree: CBTree::init(&meta.tree_name),
        }
    }
}

fn check_name(expected: &str, actual: &str) -> bool {
    !expected.is_empty() && expected == actual
}

// parse int safely
fn parse_i32(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

fn parse_isize(s: &str) -> Option<isize> {
    s.parse::<isize>().ok()
}

pub fn execute_command(c: &mut Containers, pc: &ParsedCommand) -> CommandResult {
    let t = &pc.tokens;
    if t.is_empty() {
        return make_error("ERROR: empty command");
    }
    let cmd = t[0].as_str();

    if cmd == "PRINT" {
        if t.len() < 2 {
            return make_error("ERROR: PRINT requires name");
        }
        let name = &t[1];
        if name == &c.arr.name { return c.arr.print(); }
        if name == &c.slist.name { return c.slist.print(); }
        if name == &c.dlist.name { return c.dlist.print(); }
        if name == &c.stk.name { return c.stk.print(); }
        if name == &c.q.name { return c.q.print(); }
        if name == &c.tree.name { return c.tree.print(); }
        return make_error("ERROR: unknown container name");
    }

    match pc.container_type {
        ContainerType::Array => {
            if t.len() < 2 { return make_error("ERROR: missing name"); }
            let name = &t[1];
            if !check_name(&c.arr.name, name) { return make_error("ERROR: name mismatch"); }
            match cmd {
                "MPUSH" => {
                    if t.len() == 3 {
                        return c.arr.push(&t[2], None);
                    }
                    if t.len() == 4 {
                        let idx = match parse_isize(&t[3]) {
                            Some(v) => v,
                            None => return make_error("ERROR: bad MPUSH args"),
                        };
                        return c.arr.push(&t[2], Some(idx));
                    }
                    make_error("ERROR: bad MPUSH args")
                }
                "MDEL" => {
                    if t.len() != 3 { return make_error("ERROR: bad MDEL args"); }
                    let idx = match parse_isize(&t[2]) { Some(v) => v, None => return make_error("ERROR: bad MDEL args") };
                    c.arr.del(idx)
                }
                "MGET" => {
                    if t.len() != 3 { return make_error("ERROR: bad MGET args"); }
                    let idx = match parse_isize(&t[2]) { Some(v) => v, None => return make_error("ERROR: bad MGET args") };
                    c.arr.get(idx)
                }
                "MSET" => {
                    if t.len() != 4 { return make_error("ERROR: bad MSET args"); }
                    let idx = match parse_isize(&t[2]) { Some(v) => v, None => return make_error("ERROR: bad MSET args") };
                    c.arr.set(idx, &t[3])
                }
                "MLEN" => c.arr.len(),
                _ => make_error("ERROR: unknown array command"),
            }
        }
        ContainerType::Singly => {
            if t.len() < 2 { return make_error("ERROR: missing name"); }
            let name = &t[1];
            if !check_name(&c.slist.name, name) { return make_error("ERROR: name mismatch"); }
            match cmd {
                "FPUSH" => {
                    if t.len() == 4 { return c.slist.push(&t[2], &t[3], ""); }
                    if t.len() == 5 { return c.slist.push(&t[2], &t[3], &t[4]); }
                    make_error("ERROR: bad FPUSH args")
                }
                "FDEL" => {
                    if t.len() == 3 { return c.slist.del("", &t[2]); }
                    if t.len() == 4 { return c.slist.del(&t[2], &t[3]); }
                    make_error("ERROR: bad FDEL args")
                }
                "FGET" => c.slist.get(),
                "FFIND" => {
                    if t.len() != 3 { return make_error("ERROR: bad FFIND args"); }
                    c.slist.find(&t[2])
                }
                _ => make_error("ERROR: unknown singly command"),
            }
        }
        ContainerType::Doubly => {
            if t.len() < 2 { return make_error("ERROR: missing name"); }
            let name = &t[1];
            if !check_name(&c.dlist.name, name) { return make_error("ERROR: name mismatch"); }
            match cmd {
                "LPUSH" => {
                    if t.len() == 4 { return c.dlist.push(&t[2], &t[3], ""); }
                    if t.len() == 5 { return c.dlist.push(&t[2], &t[3], &t[4]); }
                    make_error("ERROR: bad LPUSH args")
                }
                "LDEL" => {
                    if t.len() == 3 { return c.dlist.del("", &t[2]); }
                    if t.len() == 4 { return c.dlist.del(&t[2], &t[3]); }
                    make_error("ERROR: bad LDEL args")
                }
                "LGET" => c.dlist.get(),
                "LFIND" => {
                    if t.len() != 3 { return make_error("ERROR: bad LFIND args"); }
                    c.dlist.find(&t[2])
                }
                _ => make_error("ERROR: unknown doubly command"),
            }
        }
        ContainerType::Stack => {
            if t.len() < 2 { return make_error("ERROR: missing name"); }
            let name = &t[1];
            if !check_name(&c.stk.name, name) { return make_error("ERROR: name mismatch"); }
            match cmd {
                "SPUSH" => {
                    if t.len() != 3 { return make_error("ERROR: bad SPUSH args"); }
                    c.stk.push(&t[2])
                }
                "SPOP" => c.stk.pop(),
                _ => make_error("ERROR: unknown stack command"),
            }
        }
        ContainerType::Queue => {
            if t.len() < 2 { return make_error("ERROR: missing name"); }
            let name = &t[1];
            if !check_name(&c.q.name, name) { return make_error("ERROR: name mismatch"); }
            match cmd {
                "QPUSH" => {
                    if t.len() != 3 { return make_error("ERROR: bad QPUSH args"); }
                    c.q.push(&t[2])
                }
                "QPOP" => c.q.pop(),
                _ => make_error("ERROR: unknown queue command"),
            }
        }
        ContainerType::Tree => {
            if t.len() < 2 { return make_error("ERROR: missing name"); }
            let name = &t[1];
            if !check_name(&c.tree.name, name) { return make_error("ERROR: name mismatch"); }
            match cmd {
                "TINSERT" => {
                    if t.len() != 3 { return make_error("ERROR: bad TINSERT args"); }
                    let v = match parse_i32(&t[2]) { Some(v) => v, None => return make_error("ERROR: bad TINSERT args") };
                    c.tree.insert(v)
                }
                "TGET" => {
                    if t.len() != 3 { return make_error("ERROR: bad TGET args"); }
                    let v = match parse_i32(&t[2]) { Some(v) => v, None => return make_error("ERROR: bad TGET args") };
                    c.tree.get(v)
                }
                "TCHECK" => c.tree.check(),
                _ => make_error("ERROR: unknown tree command"),
            }
        }
        ContainerType::Unknown => make_error("ERROR: unknown command"),
    }
}