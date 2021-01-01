use std::{collections::HashMap, sync::RwLock, path::PathBuf};

use nix::unistd::Pid;

pub struct LocalCurrentDir {
    procs: RwLock<HashMap<Pid, PathBuf>>
}

impl LocalCurrentDir {
    pub fn init() -> Self {
        Self {
            procs: RwLock::new(HashMap::new())
        }
    }
    pub fn get<'a>(&'a self, pid: Pid) -> &'a PathBuf {
        &self.procs.read().unwrap()[&pid]
    }
}