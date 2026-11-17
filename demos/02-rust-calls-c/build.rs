use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let object = out_dir.join("mathlib.o");
    let archive = out_dir.join("libmathlib.a");

    run(Command::new("cc")
        .args(["-c", "-fPIC", "c/mathlib.c", "-o"])
        .arg(&object));
    run(Command::new("ar").arg("rcs").arg(&archive).arg(&object));

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=mathlib");
    println!("cargo:rerun-if-changed=c");
}

fn run(command: &mut Command) {
    let status = command.status().expect("failed to start command");
    assert!(status.success(), "{command:?} failed");
}
