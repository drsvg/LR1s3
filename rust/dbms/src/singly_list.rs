// singly_list.rs
use crate::common::{CommandResult, make_ok, make_error};

pub struct SNode {
    pub value: String,
    pub next: Option<Box<SNode>>,
}

pub struct SinglyList {
    pub name: String,
    pub head: Option<Box<SNode>>,
    // tail как raw pointer для O(1) вставки в конец — используем safe-обёртку
    // В Rust проще хранить tail через индекс/указатель, но чтобы сохранить логику —
    // используем Option<*mut SNode>, аккуратно управляя lifetime.
    // Безопасная альтернатива: хранить только head и искать tail за O(n).
    // Здесь сохраняем производительность: используем tail как raw указатель.
    pub tail: Option<*mut SNode>,
    pub size: usize,
}

impl SinglyList {
    pub fn init(name: &str) -> Self {
        SinglyList {
            name: name.to_string(),
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.size = 0;
    }

    // helper: получить raw pointer на голову
    fn head_ptr(&mut self) -> Option<*mut SNode> {
        self.head.as_deref_mut().map(|n| n as *mut SNode)
    }

    // helper: получить raw pointer на tail
    fn tail_ptr(&mut self) -> Option<*mut SNode> {
        self.tail
    }

    pub fn push(&mut self, value: &str, mode: &str, target: &str) -> CommandResult {
        let node = Box::new(SNode {
            value: value.to_string(),
            next: None,
        });

        match mode {
            "head" => {
                let mut node = node;
                node.next = self.head.take();
                if self.tail.is_none() {
                    self.tail = Some(&mut *node as *mut SNode);
                }
                self.head = Some(node);
            }
            "tail" => {
                let mut node = node;
                let node_ptr = &mut *node as *mut SNode;
                if let Some(tail_ptr) = self.tail {
                    // SAFETY: tail_ptr указывает на живой узел в списке
                    unsafe { (*tail_ptr).next = Some(node); }
                } else {
                    self.head = Some(node);
                }
                self.tail = Some(node_ptr);
            }
            "after" => {
                if target.is_empty() {
                    return make_error("ERROR: target required for 'after' mode");
                }
                let mut node = node;
                // Ищем target
                let mut cur_ptr: Option<*mut SNode> = self.head_ptr();
                let mut found: Option<*mut SNode> = None;
                while let Some(p) = cur_ptr {
                    unsafe {
                        if (*p).value == target {
                            found = Some(p);
                            break;
                        }
                        cur_ptr = (*p).next.as_deref_mut().map(|n| n as *mut SNode);
                    }
                }
                let Some(p) = found else {
                    return make_error("ERROR: target not found");
                };
                let node_ptr = &mut *node as *mut SNode;
                unsafe {
                    node.next = (*p).next.take();
                    (*p).next = Some(node);
                    // если вставили после хвоста
                    if self.tail == Some(p) {
                        self.tail = Some(node_ptr);
                    }
                }
            }
            "before" => {
                if target.is_empty() {
                    return make_error("ERROR: target required for 'before' mode");
                }
                let mut node = node;
                // если target — голова
                let is_head_target = self.head.as_ref().map_or(false, |h| h.value == target);
                if is_head_target {
                    node.next = self.head.take();
                    self.head = Some(node);
                } else {
                    let mut cur_ptr = self.head_ptr();
                    let mut prev_ptr: Option<*mut SNode> = None;
                    let mut found_next: Option<*mut SNode> = None;
                    while let Some(p) = cur_ptr {
                        unsafe {
                            if let Some(next_ref) = (*p).next.as_deref_mut() {
                                if next_ref.value == target {
                                    prev_ptr = Some(p);
                                    found_next = Some(next_ref as *mut SNode);
                                    break;
                                }
                            }
                            cur_ptr = (*p).next.as_deref_mut().map(|n| n as *mut SNode);
                        }
                    }
                    let Some(prev) = prev_ptr else {
                        return make_error("ERROR: target not found");
                    };
                    let Some(_next) = found_next else {
                        return make_error("ERROR: target not found");
                    };
                    unsafe {
                        node.next = (*prev).next.take();
                        (*prev).next = Some(node);
                    }
                }
            }
            _ => return make_error("ERROR: unknown mode"),
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
                let mut del = self.head.take().unwrap();
                self.head = del.next.take();
                if self.head.is_none() {
                    self.tail = None;
                }
                self.size -= 1;
                make_ok(del.value)
            }
            "tail" => {
                // если один элемент
                let is_single = self.head.as_ref().map_or(false, |h| h.next.is_none());
                if is_single {
                    let del = self.head.take().unwrap();
                    self.tail = None;
                    self.size -= 1;
                    return make_ok(del.value);
                }
                // ищем предпоследний
                let mut cur = self.head.as_deref_mut().unwrap();
                while cur.next.as_ref().map_or(false, |n| n.next.is_some()) {
                    cur = cur.next.as_deref_mut().unwrap();
                }
                let mut del = cur.next.take().unwrap();
                self.tail = Some(cur as *mut SNode);
                self.size -= 1;
                make_ok(del.value)
            }
            "value" => {
                // если голова — target
                if self.head.as_ref().map_or(false, |h| h.value == arg) {
                    let mut del = self.head.take().unwrap();
                    self.head = del.next.take();
                    if self.head.is_none() {
                        self.tail = None;
                    }
                    self.size -= 1;
                    return make_ok(del.value);
                }
                let mut cur = self.head.as_deref_mut().unwrap();
                loop {
                    let next_val = cur.next.as_ref().map(|n| n.value.clone());
                    match next_val {
                        Some(v) if v == arg => {
                            let mut del = cur.next.take().unwrap();
                            cur.next = del.next.take();
                            if del.next.is_none() {
                                self.tail = Some(cur as *mut SNode);
                            }
                            self.size -= 1;
                            return make_ok(del.value);
                        }
                        Some(_) => {
                            cur = cur.next.as_deref_mut().unwrap();
                        }
                        None => return make_error("ERROR: value not found"),
                    }
                }
            }
            _ => make_error("ERROR: unknown mode"),
        }
    }

    pub fn get(&self) -> CommandResult {
        let mut items = Vec::new();
        let mut cur = self.head.as_ref();
        while let Some(n) = cur {
            items.push(n.value.clone());
            cur = n.next.as_ref();
        }
        make_ok(format!("[{}]", items.join(", ")))
    }

    pub fn find(&self, value: &str) -> CommandResult {
        let mut cur = self.head.as_ref();
        while let Some(n) = cur {
            if n.value == value {
                return make_ok("TRUE");
            }
            cur = n.next.as_ref();
        }
        make_ok("FALSE")
    }

    pub fn print(&self) -> CommandResult {
        self.get()
    }
}

impl Drop for SinglyList {
    fn drop(&mut self) {
        // Box рекурсивно удалит цепочку, tail нам не мешает
        self.tail = None;
    }
}