pub enum S {
}

impl S {
	pub fn from_regs(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> Option<S> {
		 let call = match rax {
			other => return None
		};
	return Some(call);
	}
}
