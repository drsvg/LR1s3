// command_parser.rs
use crate::common::ContainerType;

pub struct ParsedCommand {
    pub raw: String,
    pub tokens: Vec<String>,
    pub container_type: ContainerType,
    pub command: String,
}

pub fn split_tokens(s: &str) -> Vec<String> {
    s.split_whitespace().map(|t| t.to_string()).collect()
}

pub fn container_type_from_command(command: &str) -> ContainerType {
    match command.chars().next() {
        None => ContainerType::Unknown,
        Some(c) => match c {
            'M' => ContainerType::Array,
            'F' => ContainerType::Singly,
            'L' => ContainerType::Doubly,
            'S' => ContainerType::Stack,
            'Q' => ContainerType::Queue,
            'T' => ContainerType::Tree,
            _ => ContainerType::Unknown,
        },
    }
}

pub fn parse_command(query: &str) -> ParsedCommand {
    let tokens = split_tokens(query);
    let command = tokens.first().cloned().unwrap_or_default();
    let container_type = container_type_from_command(&command);
    ParsedCommand {
        raw: query.to_string(),
        tokens,
        container_type,
        command,
    }
}