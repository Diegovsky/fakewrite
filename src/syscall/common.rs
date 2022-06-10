pub struct Read {
	 pub fd: i32,
	 pub buf: * mut i8,
	 pub count: i64,
}
pub struct Write {
	 pub fd: i32,
	 pub buf: * mut i8,
	 pub count: i64,
}
pub struct Open {
	 pub filename: * mut i8,
	 pub flags: i32,
	 pub mode: i32,
}
pub struct Close {
	 pub fd: i32,
}
pub struct Execve {
	 pub filename: * mut i8,
	 pub argv: * const * const i8,
	 pub envp: * const * const i8,
}
pub struct Rename {
	 pub oldname: * mut i8,
	 pub newname: * mut i8,
}
pub struct Mkdir {
	 pub pathname: * mut i8,
	 pub mode: i32,
}
pub struct Creat {
	 pub pathname: * mut i8,
	 pub mode: i32,
}
pub struct Unlink {
	 pub pathname: * mut i8,
}
pub struct Symlink {
	 pub oldname: * mut i8,
	 pub newname: * mut i8,
}
pub struct Openat {
	 pub dfd: i32,
	 pub filename: * mut i8,
	 pub flags: i32,
	 pub mode: i32,
}
pub struct Mkdirat {
	 pub dfd: i32,
	 pub pathname: * mut i8,
	 pub mode: i32,
}
pub struct Renameat {
	 pub oldfd: i32,
	 pub oldname: * mut i8,
	 pub newfd: i32,
	 pub newname: * mut i8,
}

pub enum SystemCall {
	Read(Read),
	Write(Write),
	Open(Open),
	Close(Close),
	Fork,
	Vfork,
	Execve(Execve),
	Rename(Rename),
	Mkdir(Mkdir),
	Creat(Creat),
	Unlink(Unlink),
	Symlink(Symlink),
	Openat(Openat),
	Mkdirat(Mkdirat),
	Renameat(Renameat),
}

