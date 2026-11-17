use std::{env, path::PathBuf};

fn main() {
    // The `cc` crate replaces our hand-rolled `cc -c` and `ar rcs`.
    cc::Build::new().file("c/mathlib.c").compile("mathlib");

    // bindgen reads the header via libclang and writes Rust declarations.
    let bindings = bindgen::Builder::default()
        .header("c/mathlib.h")
        .allowlist_function("add|point_.*")
        .allowlist_type("Point")
        .derive_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("unable to write bindings");
}
