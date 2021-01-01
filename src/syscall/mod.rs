use std::path::{Path, PathBuf};

use nix::fcntl::{self};
use nix::sys::ptrace;
use nix::unistd::{Pid};

use anyhow::{Context, Result};

use crate::operations::*;
use crate::ustring::*;
// use crate::mylog::prelude::*;

mod system;

include!(concat!(env!("OUT_DIR"), "/systemcalls.rs"));

#[macro_export]
macro_rules! catch_exit {
    ($res:expr) => {
        match $res {
            Err(Error::Sys(Errno::ESRCH)) => {
                break;
            }
            Ok(val) => val,
            Err(e) => return Err(anyhow::anyhow!("{:?}", e)),
        }
    };
}


pub fn write_syscall(info: Write, pid: Pid, oplog: &mut OperationLogger) -> Result<()> {
    let fname = filename_from_fd(info.fd, pid)?;
    let buf = DataIterator::new(info.buf as *const u8, info.count as usize, pid).collect();
    let text = String::from_utf8(buf).ok();
    oplog.log(fname, Op::Write(text));
    Ok(())
}

pub fn creat_syscall(info: Creat, pid: Pid, oplog: &mut OperationLogger) {
    oplog.log(read_string_from_proc(info.pathname as *const u8, pid), Op::Created);
}
pub fn open_syscall(info: Open, pid: Pid, oplog: &mut OperationLogger) {
    let fpath = read_string_from_proc(info.filename as *const u8, pid);
    if !fpath.as_path().exists() {
        oplog.log(fpath, Op::Created);
    }
}
pub fn openat_syscall(info: Openat, pid: Pid, oplog: &mut OperationLogger) {

}
/// This function handles the `unlink` Linux system call
pub(crate) fn unlink_syscall(unlink: Unlink, pid: Pid, logger: &mut OperationLogger) -> Result<()> {
    let filename = read_string_from_proc(unlink.pathname as *const u8, pid);
    filename.as_path().canonicalize()
        .map(|p| logger.log(filename, Op::Deleted))?;
    Ok(())
}

use std::ffi::OsStr;

pub(crate) fn solve_path<P: AsRef<Path>>(path: P) -> anyhow::Result<PathBuf> {
    let mut path = path.as_ref().to_path_buf();
    if !path.is_absolute() {
        let mut cwd = std::env::current_dir()?;
        cwd.push(path);
        path = cwd;
    }
    let mut buf = PathBuf::with_capacity(path.capacity());
    for part in path.iter() {
        if part == OsStr::new("..") {
            buf.pop();
            continue;
        } else if part == OsStr::new(".") {
            continue;
        } else {
            buf.push(part)
        }
    }
    buf.shrink_to_fit();
    Ok(buf)
}

struct DataIterator {
    pid: Pid,
    ptr: *const u8,
    value: usize,
    index: usize,
    size: Option<usize>
}
impl DataIterator {
    const WORD_SIZE: usize = std::mem::size_of::<usize>();
    pub fn new(ptr: *const u8, size: impl Into<Option<usize>>, pid: Pid) -> DataIterator {
        Self {
            pid,
            ptr: ptr,
            index: 0,
            value: 0,
            size: size.into()
        }
    }
}
impl Iterator for DataIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.size.map(|s| s <= self.index) == Some(true){
            return None
        }
        let i = self.index % Self::WORD_SIZE;
        if i == 0 {
            unsafe {
                self.value = ptrace::read(self.pid, self.ptr.add(self.index) as *mut _).ok()? as usize;
            }
        }
        let r = (self.value >> 8*i) as u8;
        self.index += 1;
        Some(r)
    }
}

pub fn read_string_from_proc(ptr: *const u8, pid: Pid) -> UString {
    let mut buf: Vec<u8> = Vec::with_capacity(64);

    for byte in DataIterator::new(ptr, None, pid) {
        if byte == 0 {
            break
        } else {
            buf.push(byte);
        }
    }
    UString::from_vec(buf)
}


pub fn filename_from_fd<'a>(fd: i32, pid: Pid) -> Result<UString> {
    fcntl::readlink(format!("/proc/{}/fd/{}", pid, fd).as_str())
        .map(UString::from_os_str)
        .context("Couldn't get filename from file descriptor.")
}
