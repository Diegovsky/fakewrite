
use super::common::*;

impl SystemCall {
	pub fn from_regs(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> Option<SystemCall> {
		 let call = match rax {
			0 => SystemCall::Read(Read{fd: rdi as i32, buf: rsi as * mut i8, count: rdx as i64}),
			1 => SystemCall::Write(Write{fd: rdi as i32, buf: rsi as * mut i8, count: rdx as i64}),
			2 => SystemCall::Open(Open{filename: rdi as * mut i8, flags: rsi as i32, mode: rdx as i32}),
			3 => SystemCall::Close(Close{fd: rdi as i32}),
			57 => SystemCall::Fork,
			58 => SystemCall::Vfork,
			59 => SystemCall::Execve(Execve{filename: rdi as * mut i8, argv: rsi as * const * const i8, envp: rdx as * const * const i8}),
			82 => SystemCall::Rename(Rename{oldname: rdi as * mut i8, newname: rsi as * mut i8}),
			83 => SystemCall::Mkdir(Mkdir{pathname: rdi as * mut i8, mode: rsi as i32}),
			85 => SystemCall::Creat(Creat{pathname: rdi as * mut i8, mode: rsi as i32}),
			87 => SystemCall::Unlink(Unlink{pathname: rdi as * mut i8}),
			88 => SystemCall::Symlink(Symlink{oldname: rdi as * mut i8, newname: rsi as * mut i8}),
			257 => SystemCall::Openat(Openat{dfd: rdi as i32, filename: rsi as * mut i8, flags: rdx as i32, mode: r10 as i32}),
			258 => SystemCall::Mkdirat(Mkdirat{dfd: rdi as i32, pathname: rsi as * mut i8, mode: rdx as i32}),
			264 => SystemCall::Renameat(Renameat{oldfd: rdi as i32, oldname: rsi as * mut i8, newfd: rdx as i32, newname: r10 as * mut i8}),
			_ => return None
		};
	return Some(call);
	}
}
