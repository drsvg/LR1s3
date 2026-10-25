// storage.rs
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct StorageMeta {
    pub array_name: String,
    pub singly_name: String,
    pub doubly_name: String,
    pub stack_name: String,
    pub queue_name: String,
    pub tree_name: String,
}

impl Default for StorageMeta {
    fn default() -> Self {
        StorageMeta {
            array_name: String::new(),
            singly_name: String::new(),
            doubly_name: String::new(),
            stack_name: String::new(),
            queue_name: String::new(),
            tree_name: String::new(),
        }
    }
}

pub fn extract_value(line: &str, key: &str) -> String {
    let search_key = format!("\"{}\"", key);
    let kpos = match line.find(&search_key) {
        Some(p) => p,
        None => return String::new(),
    };
    let colon = match line[kpos + search_key.len()..].find(':') {
        Some(p) => kpos + search_key.len() + p,
        None => return String::new(),
    };
    let first_quote = match line[colon..].find('"') {
        Some(p) => colon + p,
        None => return String::new(),
    };
    let second_quote = match line[first_quote + 1..].find('"') {
        Some(p) => first_quote + 1 + p,
        None => return String::new(),
    };
    line[first_quote + 1..second_quote].to_string()
}

pub fn split_csv(csv: &str) -> Vec<String> {
    csv.split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn load_meta(filename: &str) -> StorageMeta {
    let mut meta = StorageMeta::default();
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return meta,
    };
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        let checks: [(&str, &mut String); 6] = [
            ("|array", &mut meta.array_name),
            ("|flist", &mut meta.singly_name),
            ("|dlist", &mut meta.doubly_name),
            ("|stack", &mut meta.stack_name),
            ("|queue", &mut meta.queue_name),
            ("|tree", &mut meta.tree_name),
        ];
        for (marker, slot) in checks {
            if line.contains(marker) {
                if let Some(pipe) = line.find('|') {
                    if let Some(quote) = line.find('"') {
                        if pipe > quote {
                            *slot = line[quote + 1..pipe].to_string();
                        }
                    }
                }
                break;
            }
        }
    }
    meta
}