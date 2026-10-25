// doubly_list.rs
use std::ptr::NonNull;
use crate::common::{CommandResult, make_ok, make_error};

pub struct DNode {
    pub value: String,
    pub prev: Option<NonNull<DNode>>,
    pub next: Option<NonNull<DNode>>,
}

pub struct DoublyList {
    pub name: String,
    pub head: Option<NonNull<DNode>>,
    pub tail: Option<NonNull<DNode>>,
    pub size: usize,
}

impl DoublyList {
    pub fn init(name: &str) -> Self {
        DoublyList {
            name: name.to_string(),
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn alloc(value: &str) -> NonNull<DNode> {
        let boxed = Box::new(DNode {
            value: value.to_string(),
            prev: None,
            next: None,
        });
        NonNull::new(Box::into_raw(boxed)).unwrap()
    }

    pub fn clear(&mut self) {
        // Освобождаем всю цепочку
        let mut cur = self.head;
        while let Some(node) = cur {
            let next = unsafe { node.as_ref().next };
            unsafe { drop(Box::from_raw(node.as_ptr())); }
            cur = next;
        }
        self.head = None;
        self.tail = None;
        self.size = 0;
    }

    pub fn push(&mut self, value: &str, mode: &str, target: &str) -> CommandResult {
        let node_ptr = Self::alloc(value);

        match mode {
            "head" => {
                unsafe {
                    (*node_ptr.as_ptr()).next = self.head;
                    if let Some(mut h) = self.head {
                        h.as_mut().prev = Some(node_ptr);
                    }
                    self.head = Some(node_ptr);
                }
                if self.tail.is_none() {
                    self.tail = Some(node_ptr);
                }
            }
            "tail" => {
                unsafe {
                    (*node_ptr.as_ptr()).prev = self.tail;
                    if let Some(mut t) = self.tail {
                        t.as_mut().next = Some(node_ptr);
                    }
                    self.tail = Some(node_ptr);
                }
                if self.head.is_none() {
                    self.head = Some(node_ptr);
                }
            }
            "after" => {
                if target.is_empty() {
                    unsafe { drop(Box::from_raw(node_ptr.as_ptr())); }
                    return make_error("ERROR: target required for 'after' mode");
                }
                let mut cur = self.head;
                let mut found = None;
                while let Some(n) = cur {
                    if unsafe { n.as_ref().value == target } {
                        found = Some(n);
                        break;
                    }
                    cur = unsafe { n.as_ref().next };
                }
                let Some(mut cur) = found else {
                    unsafe { drop(Box::from_raw(node_ptr.as_ptr())); }
                    return make_error("ERROR: target not found");
                };
                unsafe {
                    let cur_next = cur.as_ref().next;
                    (*node_ptr.as_ptr()).prev = Some(cur);
                    (*node_ptr.as_ptr()).next = cur_next;
                    if let Some(mut nxt) = cur_next {
                        nxt.as_mut().prev = Some(node_ptr);
                    } else {
                        self.tail = Some(node_ptr);
                    }
                    cur.as_mut().next = Some(node_ptr);
                }
            }
            "before" => {
                if target.is_empty() {
                    unsafe { drop(Box::from_raw(node_ptr.as_ptr())); }
                    return make_error("ERROR: target required for 'before' mode");
                }
                let mut cur = self.head;
                let mut found = None;
                while let Some(n) = cur {
                    if unsafe { n.as_ref().value == target } {
                        found = Some(n);
                        break;
                    }
                    cur = unsafe { n.as_ref().next };
                }
                let Some(mut cur) = found else {
                    unsafe { drop(Box::from_raw(node_ptr.as_ptr())); }
                    return make_error("ERROR: target not found");
                };
                unsafe {
                    let cur_prev = cur.as_ref().prev;
                    (*node_ptr.as_ptr()).next = Some(cur);
                    (*node_ptr.as_ptr()).prev = cur_prev;
                    if let Some(mut prv) = cur_prev {
                        prv.as_mut().next = Some(node_ptr);
                    } else {
                        self.head = Some(node_ptr);
                    }
                    cur.as_mut().prev = Some(node_ptr);
                }
            }
            _ => {
                unsafe { drop(Box::from_raw(node_ptr.as_ptr())); }
                return make_error("ERROR: unknown mode");
            }
        }
        self.size += 1;
        make_ok(value)
    }

    pub fn del(&mut self, arg: &str, mode: &str) -> CommandResult {
        if self.head.is_none() {
            return make_error("ERROR: list is empty");
        }

        match mode {
            "head" => {
                let del = self.head.unwrap();
                let del_value = unsafe { del.as_ref().value.clone() };
                let next = unsafe { del.as_ref().next };
                self.head = next;
                if let Some(mut n) = next {
                    unsafe { n.as_mut().prev = None; }
                } else {
                    self.tail = None;
                }
                unsafe { drop(Box::from_raw(del.as_ptr())); }
                self.size -= 1;
                make_ok(del_value)
            }
            "tail" => {
                let del = self.tail.unwrap();
                let del_value = unsafe { del.as_ref().value.clone() };
                let prev = unsafe { del.as_ref().prev };
                self.tail = prev;
                if let Some(mut p) = prev {
                    unsafe { p.as_mut().next = None; }
                } else {
                    self.head = None;
                }
                unsafe { drop(Box::from_raw(del.as_ptr())); }
                self.size -= 1;
                make_ok(del_value)
            }
            "value" => {
                let mut cur = self.head;
                let mut found = None;
                while let Some(n) = cur {
                    if unsafe { n.as_ref().value == arg } {
                        found = Some(n);
                        break;
                    }
                    cur = unsafe { n.as_ref().next };
                }
                let Some(cur) = found else {
                    return make_error("ERROR: value not found");
                };
                unsafe {
                    let prev = cur.as_ref().prev;
                    let next = cur.as_ref().next;
                    if let Some(mut p) = prev {
                        p.as_mut().next = next;
                    } else {
                        self.head = next;
                    }
                    if let Some(mut n) = next {
                        n.as_mut().prev = prev;
                    } else {
                        self.tail = prev;
                    }
                    let v = cur.as_ref().value.clone();
                    drop(Box::from_raw(cur.as_ptr()));
                    self.size -= 1;
                    make_ok(v)
                }
            }
            _ => make_error("ERROR: unknown mode"),
        }
    }

    pub fn get(&self) -> CommandResult {
        let mut items = Vec::new();
        let mut cur = self.head;
        while let Some(n) = cur {
            unsafe {
                items.push(n.as_ref().value.clone());
                cur = n.as_ref().next;
            }
        }
        make_ok(format!("[{}]", items.join(", ")))
    }

    pub fn find(&self, value: &str) -> CommandResult {
        let mut cur = self.head;
        while let Some(n) = cur {
            if unsafe { n.as_ref().value == value } {
                return make_ok("TRUE");
            }
            cur = unsafe { n.as_ref().next };
        }
        make_ok("FALSE")
    }

    pub fn print(&self) -> CommandResult {
        self.get()
    }
}

impl Drop for DoublyList {
    fn drop(&mut self) {
        self.clear();
    }
}