#![allow(unused_imports)]
use crate::ustring::UString;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::path::Path;

#[test]
fn ustring_as_str() {
    let st = "This is a test";
    let ust = UString::from(st);
    assert_eq!(ust.as_str(), st);
}
#[test]
fn ustring_as_c_str() {
    let st = "This is a test";
    let ust = UString::from(st);
    assert_eq!(ust.as_c_str(), CString::new(st).unwrap().as_ref());
}

#[test]
fn ustring_as_os_str() {
    let st = "This is a test";
    let ust = UString::from(st);
    assert_eq!(ust.as_os_str(), OsStr::new(st));
}
