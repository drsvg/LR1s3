// stack.rs
use crate::common::{CommandResult, make_ok, make_error};

pub struct StackNode {
    pub value: String,
    pub next: Option<Box<StackNode>>,
}

pub struct Stack {
    pub name: String,
    pub top: Option<Box<StackNode>>,
    pub size: usize,
}

impl Stack {
    pub fn init(name: &str) -> Self {
        Stack {
            name: name.to_string(),
            top: None,
            size: 0,
        }
    }

    pub fn clear(&mut self) {
        self.top = None;
        self.size = 0;
    }

    pub fn push(&mut self, value: &str) -> CommandResult {
        let node = Box::new(StackNode {
            value: value.to_string(),
            next: self.top.take(),
        });
        self.top = Some(node);
        self.size += 1;
        make_ok(value)
    }

    pub fn pop(&mut self) -> CommandResult {
        match self.top.take() {
            None => make_error("ERROR: stack is empty"),
            Some(mut del) => {
                self.top = del.next.take();
                self.size -= 1;
                make_ok(del.value)
            }
        }
    }

    pub fn print(&self) -> CommandResult {
        let mut items = Vec::new();
        let mut cur = self.top.as_ref();
        while let Some(n) = cur {
            items.push(n.value.clone());
            cur = n.next.as_ref();
        }
        make_ok(format!("[{}]", items.join(", ")))
    }
}