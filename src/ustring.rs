use std::borrow::Cow;
use std::ffi::{CStr, OsStr};
use std::path::Path;

use std::mem;

#[derive(Clone, Eq, PartialEq, Hash)]
/// A String type to help with type conversion between the string types.
/// # Fields
/// * `buf` Buf always ends with a trailing 0 to costlessly borrow as CStr.
/// Note: Both the `&str` and `String` conversions exclude the traling 0.
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

    /// If the contents are all valid UTF-8, it's basically a no-op (see [`from_vec_unchecked`]).
    /// Otherwise, performs a lossy conversion of the bytes into UTF-8.
    pub fn from_vec(buf: Vec<u8>) -> Self {
        match String::from_utf8(buf) {
            Ok(buf) => unsafe { Self::from_vec_unchecked(buf.into_bytes()) }
            Err(e) => Self::from_slice(e.as_bytes()),
        }
    }
    /// This function assumes the content is UTF-8 encoded and appends a 0 to it.
    /// This does not check for UTF-8 validity, so you should use [`from_vec`] instead.
    pub unsafe fn from_vec_unchecked(mut buf: Vec<u8>) -> Self {
        buf.push(0);
        Self { buf }
    }

    /// Performs a lossy conversion of the slice into UTF-8
    pub fn from_slice(slc: &[u8]) -> Self {
        let st = String::from_utf8_lossy(slc).into_owned().into_bytes();
        unsafe { Self::from_vec_unchecked(st) }
    }

    /// This function gets a UTF-8 slice and appends a 0 to it.
    /// Note that it does not check for UTF-8 validity.
    unsafe fn from_slice_unchecked(slc: &[u8]) -> Self {
        let len = slc.len();

        let mut ust = Self::with_capacity(len + 1);
        ust.buf.resize(len + 1, 0);
        ust.buf[0..len].copy_from_slice(slc);
        ust
    }
    pub fn from_os_str<O: AsRef<OsStr>>(ost: O) -> Self {
        let ost = ost.as_ref();
        let st = ost.to_string_lossy().into_owned();
        unsafe { Self::from_vec_unchecked(st.into_bytes()) }
    }
    pub fn from_c_str<C: AsRef<CStr>>(cst: C) -> Self {
        let cst = cst.as_ref();
        let st = cst.to_string_lossy().into_owned();
        unsafe { Self::from_vec_unchecked(st.into_bytes()) }
    }
    pub fn from_str<C: AsRef<str>>(st: C) -> Self {
        let st = st.as_ref();
        unsafe { Self::from_slice_unchecked(st.as_bytes()) }
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
        Path::new(self.as_os_str())
    }
}

macro_rules! impl_as_ref {
    ($method:ident, $name:ident) => {
        impl AsRef<$name> for UString {
            fn as_ref(&self) -> &$name {
                self.$method()
            }
        }
    };
}

macro_rules! impl_from {
    ($method:ident, $name:ident) => {
        impl From<&$name> for UString {
            fn from(st: &$name) -> Self {
                Self::$method(st)
            }
        }
    };
}

macro_rules! impl_conversion {
    ($from:ident, $as_ref:ident, $name:ident) => {
        impl_from!($from, $name);
        impl_as_ref!($as_ref, $name);
    };
    ($($from:ident, $as_ref:ident, $name:ident);*) => {
        $(impl_conversion!($from, $as_ref, $name);)*
    }
}

// Thanks to this macro, implementing conversions is made very easy.
impl_conversion! {
    from_c_str,   as_c_str,   CStr;
    from_os_str,  as_os_str,  OsStr;
    from_str,     as_str,     str
}

use std::fmt;
use std::str::Utf8Error;

impl fmt::Debug for UString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
