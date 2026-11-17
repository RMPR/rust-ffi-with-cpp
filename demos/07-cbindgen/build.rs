use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(&crate_dir)
        .expect("unable to generate header")
        .write_to_file(format!("{crate_dir}/include/rusty2.h"));

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=cbindgen.toml");
}
