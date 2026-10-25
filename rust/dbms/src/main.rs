// main.rs
mod common;
mod array;
mod singly_list;
mod doubly_list;
mod stack;
mod queue;
mod cbtree;
mod command_parser;
mod storage;
mod dispatcher;

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::collections::VecDeque;

use command_parser::parse_command;
use dispatcher::{Containers, execute_command};
use storage::{load_meta, extract_value, split_csv, StorageMeta};
use cbtree::TNode;

fn load_containers(c: &mut Containers, filename: &str) {
    let content = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(_) => return,
    };
    for line in content.lines() {
        if line.contains("|array") {
            let key = format!("{}|array", c.arr.name);
            let csv = extract_value(line, &key);
            for v in split_csv(&csv) {
                c.arr.push(&v, None);
            }
        } else if line.contains("|flist") {
            let key = format!("{}|flist", c.slist.name);
            let csv = extract_value(line, &key);
            for v in split_csv(&csv) {
                c.slist.push(&v, "tail", "");
            }
        } else if line.contains("|dlist") {
            let key = format!("{}|dlist", c.dlist.name);
            let csv = extract_value(line, &key);
            for v in split_csv(&csv) {
                c.dlist.push(&v, "tail", "");
            }
        } else if line.contains("|stack") {
            let key = format!("{}|stack", c.stk.name);
            let csv = extract_value(line, &key);
            for v in split_csv(&csv) {
                c.stk.push(&v);
            }
        } else if line.contains("|queue") {
            let key = format!("{}|queue", c.q.name);
            let csv = extract_value(line, &key);
            for v in split_csv(&csv) {
                c.q.push(&v);
            }
        } else if line.contains("|tree") {
            let key = format!("{}|tree", c.tree.name);
            let csv = extract_value(line, &key);
            for v in split_csv(&csv) {
                if let Ok(n) = v.parse::<i32>() {
                    c.tree.insert(n);
                }
            }
        }
    }
}

fn save_containers(c: &Containers, filename: &str, meta: &StorageMeta) {
    let mut out = match File::create(filename) {
        Ok(f) => f,
        Err(_) => return,
    };

    let mut s = String::new();
    s.push_str("{\n");

    s.push_str(&format!("    \"{}|array\": \"", meta.array_name));
    for v in &c.arr.data {
        s.push_str(v);
        s.push(',');
    }
    s.push_str("\",\n");

    s.push_str(&format!("    \"{}|flist\": \"", meta.singly_name));
    let mut cur = c.slist.head.as_ref();
    while let Some(n) = cur {
        s.push_str(&n.value);
        s.push(',');
        cur = n.next.as_ref();
    }
    s.push_str("\",\n");

    s.push_str(&format!("    \"{}|dlist\": \"", meta.doubly_name));
    let mut cur = c.dlist.head;
    while let Some(n) = cur {
        unsafe {
            s.push_str(&n.as_ref().value);
            s.push(',');
            cur = n.as_ref().next;
        }
    }
    s.push_str("\",\n");

    s.push_str(&format!("    \"{}|stack\": \"", meta.stack_name));
    let mut stk_items = Vec::new();
    let mut cur = c.stk.top.as_ref();
    while let Some(n) = cur {
        stk_items.push(n.value.clone());
        cur = n.next.as_ref();
    }
    for v in stk_items.iter().rev() {
        s.push_str(v);
        s.push(',');
    }
    s.push_str("\",\n");

    s.push_str(&format!("    \"{}|queue\": \"", meta.queue_name));
    let mut cur = c.q.front;
    while let Some(n) = cur {
        unsafe {
            s.push_str(&n.as_ref().value);
            s.push(',');
            cur = n.as_ref().next;
        }
    }
    s.push_str("\",\n");

    s.push_str(&format!("    \"{}|tree\": \"", meta.tree_name));
    if let Some(root) = c.tree.root.as_ref() {
        let mut qq: VecDeque<&TNode> = VecDeque::new();
        qq.push_back(root);
        while let Some(cur) = qq.pop_front() {
            s.push_str(&cur.value.to_string());
            s.push(',');
            if let Some(l) = cur.left.as_ref() { qq.push_back(l); }
            if let Some(r) = cur.right.as_ref() { qq.push_back(r); }
        }
    }
    s.push_str("\"\n}");

    let _ = out.write_all(s.as_bytes());
}

fn is_mutating(cmd: &str) -> bool {
    matches!(cmd,
        "MPUSH" | "MDEL" | "MSET" |
        "FPUSH" | "FDEL" |
        "LPUSH" | "LDEL" |
        "SPUSH" | "SPOP" |
        "QPUSH" | "QPOP" |
        "TINSERT"
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = String::new();
    let mut query = String::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--file" && i + 1 < args.len() {
            filename = args[i + 1].clone();
            i += 2;
        } else if args[i] == "--query" && i + 1 < args.len() {
            query = args[i + 1].clone();
            i += 2;
        } else {
            i += 1;
        }
    }

    if filename.is_empty() || query.is_empty() {
        println!("ERROR: usage: ./dbms --file <file> --query '<command>'");
        std::process::exit(1);
    }

    let mut meta = load_meta(&filename);

    if meta.array_name.is_empty() { meta.array_name = "myarray".to_string(); }
    if meta.singly_name.is_empty() { meta.singly_name = "mylist".to_string(); }
    if meta.doubly_name.is_empty() { meta.doubly_name = "mydlist".to_string(); }
    if meta.stack_name.is_empty() { meta.stack_name = "mystack".to_string(); }
    if meta.queue_name.is_empty() { meta.queue_name = "myqueue".to_string(); }
    if meta.tree_name.is_empty() { meta.tree_name = "mytree".to_string(); }

    let mut c = Containers::init(&meta);
    load_containers(&mut c, &filename);

    let pc = parse_command(&query);
    let res = execute_command(&mut c, &pc);

    if res.ok && is_mutating(&pc.command) {
        save_containers(&c, &filename, &meta);
    }

    println!("{}", res.output);

    std::process::exit(if res.ok { 0 } else { 1 });
}
