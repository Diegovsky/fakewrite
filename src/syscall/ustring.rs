use std::borrow::Cow;
use std::ffi::{CStr, OsStr};
use std::path::Path;

use std::mem;

#[derive(Clone, Eq, PartialEq, Hash)]
/// A String type to help with type conversion between the string types.
/// # Fields
/// * `buf` Buf always ends with a trailing 0 to costlessly borrow as CStr.
/// # Examples
/// ```
/// let cst = CString::new("ustring.txt").unwrap();
/// let ust = UString::from(&cst);  
/// assert_eq!(Path::new("ustring.txt"), ust.as_ref());
/// ```
pub struct UString {
    buf: Vec<u8>,
}

#[allow(dead_code)]
impl UString {
    /// Allocates an empty UString.
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(8),
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: Vec::with_capacity(cap),
        }
    }
    /// This function gets a UTF-8 bytes vec and appends a 0 to it.
    /// As with [`from_slice`], this does not check for UTF-8 validity.
    pub fn from_vec(mut buf: Vec<u8>) -> Self {
        buf.push(0);
        Self { buf: buf }
    }
    /// This function gets a UTF-8 slice and appends a 0 to it.
    /// Note that it does not check for validity.
    fn from_slice(slc: &[u8]) -> Self {
        let len = slc.len();

        let mut ust = Self::with_capacity(len + 1);
        ust.buf.resize(len + 1, 0);
        ust.buf[0..len].copy_from_slice(slc);
        ust
    }
    pub fn from_os_str<O: AsRef<OsStr>>(ost: O) -> Self {
        let ost = ost.as_ref();
        let st = ost.to_string_lossy().into_owned();
        Self {
            buf: st.into_bytes(),
        }
    }
    pub fn from_c_str<C: AsRef<CStr>>(cst: C) -> Self {
        let cst = cst.as_ref();
        let st = cst.to_string_lossy().into_owned();
        Self {
            buf: st.into_bytes(),
        }
    }
    pub fn from_str<C: AsRef<str>>(st: C) -> Self {
        let st = st.as_ref();
        Self::from_slice(st.as_bytes())
    }
    pub fn as_c_str(&self) -> &CStr {
        // This is "safe" because both types have the same memory layout.
        unsafe { mem::transmute(&*self.buf) }
    }
    pub fn as_os_str(&self) -> &OsStr {
        // This is "safe" because both types have the same memory layout.
        unsafe { mem::transmute(&self.buf[0..self.buf.len() - 1]) }
    }
    pub fn as_str(&self) -> &str {
        // This is "safe" because both types have the same memory layout.
        unsafe { mem::transmute(&self.buf[0..self.buf.len() - 1]) }
    }
    pub fn as_path(&self) -> &Path {
        // This is "safe" because both types have the same memory layout.
        unsafe { mem::transmute(self.as_os_str()) }
    }
}
// Trait implementations of UString::from_x_str
impl From<&OsStr> for UString {
    fn from(ost: &OsStr) -> Self {
        Self::from_os_str(ost)
    }
}
impl From<&CStr> for UString {
    fn from(cst: &CStr) -> Self {
        Self::from_c_str(cst)
    }
}
impl From<&str> for UString {
    fn from(st: &str) -> Self {
        Self::from_str(st)
    }
}
impl From<&Path> for UString {
    fn from(p: &Path) -> Self {
        Self::from_os_str(p)
    }
}

impl AsRef<Path> for UString {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

use std::fmt::{self};

impl fmt::Debug for UString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
