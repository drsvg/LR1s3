// cbtree.rs
use std::collections::VecDeque;
use crate::common::{CommandResult, make_ok, make_error};

pub struct TNode {
    pub value: i32,
    pub left: Option<Box<TNode>>,
    pub right: Option<Box<TNode>>,
}

pub struct CBTree {
    pub name: String,
    pub root: Option<Box<TNode>>,
    pub size: usize,
}

impl CBTree {
    pub fn init(name: &str) -> Self {
        CBTree {
            name: name.to_string(),
            root: None,
            size: 0,
        }
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }

    pub fn insert(&mut self, value: i32) -> CommandResult {
        let node = Box::new(TNode {
            value,
            left: None,
            right: None,
        });

        if self.root.is_none() {
            self.root = Some(node);
            self.size += 1;
            return make_ok(value.to_string());
        }

        // BFS: ищем первый узел без левого или правого ребёнка
        // Используем очередь raw-указателей, чтобы иметь возможность модифицировать
        let root_ptr: *mut TNode = self.root.as_deref_mut().unwrap() as *mut TNode;
        let mut q: VecDeque<*mut TNode> = VecDeque::new();
        q.push_back(root_ptr);
        while let Some(cur) = q.pop_front() {
            unsafe {
                if (*cur).left.is_none() {
                    (*cur).left = Some(node);
                    self.size += 1;
                    return make_ok(value.to_string());
                }
                if (*cur).right.is_none() {
                    (*cur).right = Some(node);
                    self.size += 1;
                    return make_ok(value.to_string());
                }
                let l = (*cur).left.as_deref_mut().unwrap() as *mut TNode;
                let r = (*cur).right.as_deref_mut().unwrap() as *mut TNode;
                q.push_back(l);
                q.push_back(r);
            }
        }
        make_error("ERROR: insertion failed")
    }

    pub fn get(&self, value: i32) -> CommandResult {
        fn find(n: &Option<Box<TNode>>, value: i32) -> bool {
            match n {
                None => false,
                Some(node) => {
                    node.value == value || find(&node.left, value) || find(&node.right, value)
                }
            }
        }
        make_ok(if find(&self.root, value) { "TRUE" } else { "FALSE" })
    }

    pub fn check(&self) -> CommandResult {
        fn is_complete(n: &Option<Box<TNode>>, index: usize, total: usize) -> bool {
            match n {
                None => true,
                Some(_) => {
                    if index >= total {
                        return false;
                    }
                    true // проверка рекурсивно ниже
                }
            }
        }
        // реализуем напрямую
        fn check_rec(n: &Option<Box<TNode>>, index: usize, total: usize) -> bool {
            match n {
                None => true,
                Some(node) => {
                    if index >= total {
                        return false;
                    }
                    check_rec(&node.left, 2 * index + 1, total)
                        && check_rec(&node.right, 2 * index + 2, total)
                }
            }
        }
        let _ = is_complete; // избегаем warning
        let ok = check_rec(&self.root, 0, self.size);
        make_ok(if ok { "TRUE" } else { "FALSE" })
    }

    pub fn print(&self) -> CommandResult {
        fn collect(n: &Option<Box<TNode>>, out: &mut Vec<String>) {
            if let Some(node) = n {
                out.push(node.value.to_string());
                collect(&node.left, out);
                collect(&node.right, out);
            }
        }
        let mut items = Vec::new();
        collect(&self.root, &mut items);
        make_ok(format!("[{}]", items.join(" ")))
    }
}