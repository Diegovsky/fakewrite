// extern crate bindgen;

use std::env;
use std::path::PathBuf;

use std::process::Command;

fn main() {
    let build_dir = PathBuf::from("build");

    // Invalidate build if the python script changed.
    println!("cargo:rerun-if-changed=tools/parse.py");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let calls = [
        "write", "fork", "vfork", "execve", "rename", "mkdir", "creat", "open", "openat", "chdir",
        "unlink", "symlink", "renameat",
    ];

    let mut result = Command::new("python3")
        .arg("tools/parse.py")
        .arg("SystemCall")
        .arg(out_path.join("systemcalls.rs").to_str().unwrap())
        .args(&calls)
        .spawn()
        .expect("Could not generate syscall implementation");
    
    result.wait().unwrap();
}
