use crate::syscall::UString;
use std::collections::HashMap;

use anyhow::Result;
pub use log::{log, warn, trace, debug};

pub fn init_logger(debug: bool) -> Result<()> {
    fern::Dispatch::new()
    .format(|out, msg, record| {
        out.finish(format_args!("[{}] {}", record.level(), msg))
    })
    .level(if debug {log::LevelFilter::max()} else {log::LevelFilter::Off})
    .chain(std::io::stderr())
    .apply()?;
    Ok(())
}

pub struct OperationLogger(HashMap<UString, Vec<Op>>);

impl OperationLogger {
    pub fn new() -> Self {
        OperationLogger(HashMap::new())
    }
    pub fn log(&mut self, key: UString, op: Op) {
        if let Some(val) = self.0.get_mut(&key) {
            val.push(op)
        } else {
            self.0.insert(key, vec![op]);
        }
    }
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&UString, &Vec<Op>)> {
        self.0.iter()
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Linked(UString),
    Write,
    Created,
    Deleted,
}

use std::fmt;

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Op::Linked(ref path) = self {
            write!(f, "Symlinked at {:?}", path)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
