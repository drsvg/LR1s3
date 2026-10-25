// queue.rs
use std::ptr::NonNull;
use crate::common::{CommandResult, make_ok, make_error};

pub struct QueueNode {
    pub value: String,
    pub next: Option<NonNull<QueueNode>>,
}

pub struct Queue {
    pub name: String,
    pub front: Option<NonNull<QueueNode>>,
    pub rear: Option<NonNull<QueueNode>>,
    pub size: usize,
}

impl Queue {
    pub fn init(name: &str) -> Self {
        Queue {
            name: name.to_string(),
            front: None,
            rear: None,
            size: 0,
        }
    }

    pub fn clear(&mut self) {
        let mut cur = self.front;
        while let Some(node) = cur {
            let next = unsafe { node.as_ref().next };
            unsafe { drop(Box::from_raw(node.as_ptr())); }
            cur = next;
        }
        self.front = None;
        self.rear = None;
        self.size = 0;
    }

    pub fn push(&mut self, value: &str) -> CommandResult {
        let boxed = Box::new(QueueNode {
            value: value.to_string(),
            next: None,
        });
        let node_ptr = NonNull::new(Box::into_raw(boxed)).unwrap();
        match self.rear {
            None => {
                self.front = Some(node_ptr);
                self.rear = Some(node_ptr);
            }
            Some(mut r) => {
                unsafe { r.as_mut().next = Some(node_ptr); }
                self.rear = Some(node_ptr);
            }
        }
        self.size += 1;
        make_ok(value)
    }

    pub fn pop(&mut self) -> CommandResult {
        match self.front {
            None => make_error("ERROR: queue is empty"),
            Some(del) => {
                unsafe {
                    let v = del.as_ref().value.clone();
                    let next = del.as_ref().next;
                    self.front = next;
                    if next.is_none() {
                        self.rear = None;
                    }
                    drop(Box::from_raw(del.as_ptr()));
                    self.size -= 1;
                    make_ok(v)
                }
            }
        }
    }

    pub fn print(&self) -> CommandResult {
        let mut items = Vec::new();
        let mut cur = self.front;
        while let Some(n) = cur {
            unsafe {
                items.push(n.as_ref().value.clone());
                cur = n.as_ref().next;
            }
        }
        make_ok(format!("[{}]", items.join(", ")))
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        self.clear();
    }
}