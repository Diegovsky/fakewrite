use std::os::unix::io::RawFd;
use std::path::{Path, PathBuf};

use nix::fcntl::{self, OFlag};
use nix::sys::ptrace;
use nix::unistd::{self, Pid};

use anyhow::{Context, Result};

pub mod consts;
mod ustring;

pub use crate::logs::*;
pub use ustring::*;

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

pub fn read_cstr_in_process(ptr: u64, pid: Pid) -> UString {
    let ptr = ptr as *const u8;
    const PROB_MAX_PATH: usize = 256;
    const SIZE: usize = std::mem::size_of::<usize>();

    // Allocating a buffer to read the cstring.
    let mut buf: Vec<u8> = Vec::with_capacity(PROB_MAX_PATH);
    'main: for i in (0..PROB_MAX_PATH).step_by(SIZE) {
        let word = unsafe {
            // The program will never write memory, so this cast is safe
            match ptrace::read(pid, ptr.offset(i as isize) as *mut _) {
                Ok(val) => val as usize,
                Err(_) => break,
            }
        };
        for j in 0..SIZE {
            let byte = (word >> (j * 8)) as u8;
            if byte == 0 {
                break 'main;
            }
            buf.push(byte);
        }
    }
    buf.shrink_to_fit();
    UString::from_vec(buf)
}

pub fn from_fd<'a>(fd: i64, pid: Pid) -> Result<UString> {
    fcntl::readlink(format!("/proc/{}/fd/{}", pid, fd).as_str())
        .map(UString::from_os_str)
        .context("Couldn't get filename from file descriptor.")
}

pub(crate) fn open_call(file: u64, flags: u64, pid: Pid, oplog: &mut OperationLogger) -> Result<()> {
    let flags = flags as i32;
    if flags != OFlag::O_RDONLY.bits() {
        let path = &solve_path(read_cstr_in_process(file, pid).as_path())?;
        if (flags & (OFlag::O_APPEND.bits() | OFlag::O_CREAT.bits())) != 0 {
            match unistd::access(path, unistd::AccessFlags::F_OK) {
                Ok(_) => (),
                Err(_) => oplog.log(UString::from(path.as_path()), Op::Created),
            }
        }
    }
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

/// This function handles the `unlink` Linux system call
pub(crate) fn unlink(filename: u64, pid: Pid, logger: &mut OperationLogger) -> Result<()> {
    let filename = read_cstr_in_process(filename, pid);
    let path = filename.as_path();
    path.canonicalize()
        .map(|p| logger.log(UString::from(p.as_path()), Op::Deleted))?;
    Ok(())
}

pub(crate) fn openat_call(
    fd: u64,
    filename: u64,
    flags: u64,
    pid: Pid,
    logger: &mut OperationLogger,
) -> Result<()> {
    let flags = flags as i32;
    let filename = read_cstr_in_process(filename as u64, pid);

    if flags != OFlag::O_RDONLY.bits() {
        let mut path = PathBuf::from(filename.as_os_str());
        if fd as i32 == consts::AT_FDCWD {
            path = std::env::current_dir()?.join(&path);
        } else {
            from_fd(fd as i64, pid).map(|rel_path| rel_path.as_path().join(&path));
        }
        if flags & (OFlag::O_CREAT).bits() != 0 {
            match unistd::access(&path, unistd::AccessFlags::F_OK) {
                Ok(_) => (),
                Err(_) => logger.log(UString::from(path.as_path()), Op::Created),
            };
        }
    }
    Ok(())
}
