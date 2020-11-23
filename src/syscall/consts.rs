// Include the AT_FDCWD Linux constant defined at fnctl.h.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//const AT_FDCWD: i32;
