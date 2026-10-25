// array.rs
use crate::common::{CommandResult, make_ok, make_error};

pub const MAX_ARRAY_SIZE: usize = 10000;

pub struct Array {
    pub name: String,
    pub data: Vec<String>,
}

impl Array {
    pub fn init(name: &str) -> Self {
        Array {
            name: name.to_string(),
            data: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn push(&mut self, value: &str, index: Option<isize>) -> CommandResult {
        if self.data.len() >= MAX_ARRAY_SIZE {
            return make_error("ERROR: array overflow");
        }
        match index {
            None | Some(-1) => {
                self.data.push(value.to_string());
                make_ok(value)
            }
            Some(idx) => {
                let idx = idx as usize;
                if idx > self.data.len() {
                    return make_error("ERROR: index out of range");
                }
                self.data.insert(idx, value.to_string());
                make_ok(value)
            }
        }
    }

    pub fn del(&mut self, index: isize) -> CommandResult {
        if index < 0 || index as usize >= self.data.len() {
            return make_error("ERROR: index out of range");
        }
        let removed = self.data.remove(index as usize);
        make_ok(removed)
    }

    pub fn get(&self, index: isize) -> CommandResult {
        if index < 0 || index as usize >= self.data.len() {
            return make_error("ERROR: index out of range");
        }
        make_ok(self.data[index as usize].clone())
    }

    pub fn set(&mut self, index: isize, value: &str) -> CommandResult {
        if index < 0 || index as usize >= self.data.len() {
            return make_error("ERROR: index out of range");
        }
        self.data[index as usize] = value.to_string();
        make_ok(value)
    }

    pub fn len(&self) -> CommandResult {
        make_ok(self.data.len().to_string())
    }

    pub fn print(&self) -> CommandResult {
        make_ok(format!("[{}]", self.data.join(", ")))
    }
}