use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut objects = Vec::new();

    for name in ["word_counter", "shim"] {
        let object = out_dir.join(format!("{name}.o"));
        run(Command::new("c++")
            .args([
                "-std=c++17",
                "-fPIC",
                "-c",
                &format!("cpp/{name}.cpp"),
                "-o",
            ])
            .arg(&object));
        objects.push(object);
    }
    let archive = out_dir.join("libwordcounter.a");
    run(Command::new("ar").arg("rcs").arg(&archive).args(&objects));

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=wordcounter");
    // The shim is C, but behind it lives C++: we need its runtime too.
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rerun-if-changed=cpp");
}

fn run(command: &mut Command) {
    let status = command.status().expect("failed to start command");
    assert!(status.success(), "{command:?} failed");
}
