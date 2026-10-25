// common.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerType {
    Array,
    Singly,
    Doubly,
    Stack,
    Queue,
    Tree,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct CommandResult {
    pub ok: bool,
    pub output: String,
}

impl CommandResult {
    pub fn ok(output: impl Into<String>) -> Self {
        CommandResult {
            ok: true,
            output: output.into(),
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        CommandResult {
            ok: false,
            output: msg.into(),
        }
    }
}

// Свободные функции — аналог makeOk / makeError из C++
pub fn make_ok(output: impl Into<String>) -> CommandResult {
    CommandResult::ok(output)
}

pub fn make_error(msg: impl Into<String>) -> CommandResult {
    CommandResult::error(msg)
}
