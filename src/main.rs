use std::ffi::{CStr, CString};

use anyhow::{Context, Result};
use nix::errno::Errno;
use nix::sys::signal::Signal;
use nix::sys::{
    ptrace::{self, Event, Options},
    wait::{self, WaitStatus},
};
use nix::unistd::{self, ForkResult};
use nix::Error;

mod logs;
mod syscall;

use logs::*;

fn tracer(pid: unistd::Pid, logger: &mut OperationLogger) -> Result<()> {
    wait::waitpid(pid, None).context("Couldn't wait for child")?;
    ptrace::setoptions(
        pid,
        Options::PTRACE_O_EXITKILL
            & Options::PTRACE_O_TRACEVFORK
            & Options::PTRACE_O_TRACEEXEC
            & Options::PTRACE_O_TRACEFORK
            & Options::PTRACE_O_TRACECLONE,
    )
    .context("Couldn't set tracee options")?;
    loop {
        catch_exit!(ptrace::syscall(pid, None));
        match wait::waitpid(pid, None)? {
            WaitStatus::PtraceEvent(pid, Signal::SIGTRAP, val) => {
                let val: Event = unsafe {
                    // Event is repr(i32) so it is fine.
                    std::mem::transmute(val)
                };
                if val == Event::PTRACE_EVENT_FORK {
                    tracer(pid, logger).context("Could not trace child of child")?;
                }
            }
            WaitStatus::PtraceEvent(_, _, _) => println!("uh oh"),
            WaitStatus::Signaled(pid, sig, bol) => println!("mood: {} {} {}", pid, sig, bol),
            _ => (),
        };
        let regs = catch_exit!(ptrace::getregs(pid));
        // println!("Syscall n°: {}", regs.orig_rax);
        match syscall::SystemCall::from_regs(regs.orig_rax, regs.rdi, regs.rsi, regs.rdx, regs.r10, regs.r8, regs.r9) {
            Some(call) => {
                println!("{:?}", std::env::current_dir())
            },
            None => (),
        };

        ptrace::syscall(pid, None).context("Couldn't resume from syscall")?;
        wait::waitpid(pid, None)?;
    }
    Ok(())
}

fn child<'a>(args: impl Iterator<Item = &'a str>) -> Result<()> {
    let mut iter = args.map(|s| CString::new(s).unwrap());
    // There will always at least one element.
    let program = iter.nth(0).unwrap();
    // Save the values so they are not dropped.
    let args: Vec<CString> = iter.collect();
    // Convert the values to &CStr.
    let cargs = args.iter().map(|s| s.as_c_str()).collect::<Vec<&CStr>>();
    ptrace::traceme()?;
    unistd::execvp(&program, cargs.as_ref())?;
    Ok(())
}

use clap::clap_app;

mod tests;

fn main() -> Result<()> {
    let matches = clap_app!(myapp =>
        (author: "Diego A.")
        (about: "Strives to do the same thing fakeroot does, except it fakes writes, file creations, deletes.")
        (@arg PROGRAM: +required +multiple)
        (@arg DEBUG: --debug "Activate debug.")
    ).get_matches();

    let mut oplog = OperationLogger::new();
    logs::init_logger(matches.is_present("DEBUG"))?;

    let program = matches.values_of("PROGRAM").unwrap(); // Can't fail.
    match unistd::fork().context("Couldn't fork")? {
        ForkResult::Child => child(program).context("Couldn't run child")?,
        ForkResult::Parent { child } => {
            tracer(child, &mut oplog).context("Couldn't run tracer")?
        }
    }
    println!("Operations recorded: ");
    for (k, v) in oplog.iter() {
        println!("{:?}:", k);
        for op in v.iter() {
            println!("\t{}", op)
        }
    }
    
    Ok(())
}
