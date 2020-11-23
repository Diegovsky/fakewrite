// extern crate bindgen;

use std::env;
use std::path::PathBuf;

use std::process::Command;

fn main() {
    let build_dir = PathBuf::from("build");

    let header = build_dir
        .join("definitions.h")
        .to_str()
        .unwrap()
        .to_string();

    let bindings = bindgen::Builder::default()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Invalidate build if the python script changed.
    println!("cargo:rerun-if-changed=tools/parse.py");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let calls = [
        "read", "write", "open", "close", "fork", "vfork", "execve", "rename", "mkdir", "creat",
        "unlink", "symlink", "openat", "mkdirat", "renameat",
    ];

    let mut result = Command::new("python3")
        .arg("tools/parse.py")
        .arg("SystemCall")
        .arg(out_path.join("systemcalls.rs").to_str().unwrap())
        .args(&calls)
        .spawn()
        .expect("Could not generate syscall implementation");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    result.wait().unwrap();
}
