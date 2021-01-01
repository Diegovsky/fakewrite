use crate::ustring::UString;
use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::Result;


pub struct OperationLogger{
    pub name: Option<String>,
    logs: HashMap<UString, Vec<Op>>,
    children: Mutex<Vec<OperationLogger>>,
}

impl OperationLogger {
    pub fn new() -> Self {
        Self {
            name: None,
            logs: HashMap::with_capacity(32),
            children: Mutex::new(Vec::new()),
        }
    }
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = Some(name.into());
    }
    pub fn fork(&mut self) -> &mut OperationLogger {
        let mut vec = self.children.lock().unwrap();
        vec.push(Self::new());
        return unsafe { std::mem::transmute(vec.as_mut_ptr().offset(-1 + vec.len() as isize)) };
    }

    pub fn log(&mut self, key: UString, op: Op) {
    if let Some(val) = self.logs.get_mut(&key) {
        val.push(op)
    } else {
        self.logs.insert(key, vec![op]);
    }
    }
    pub fn extend(&mut self, other: Self) {
        self.logs.extend(other.logs)
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, UString, Vec<Op>> {
        self.logs.iter()
    }
}
struct OpIter {
    
}

#[derive(Debug, Clone)]
pub enum Op {
    Linked(UString),
    Write(Option<String>),
    Created,
    Deleted,
}

use std::fmt;

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Op::Linked(ref path) = self {
            write!(f, "Symlinked at {:?}", path)
        } else if let Op::Write(ref val) = self {
            match val {
                Some(text) => write!(f, "Write('{}')", text),
                None => write!(f, "Write"),
            }
        } else {
            write!(f, "{:?}", self)
        }
    }
}
