fn main() {
    // The very same C++ class as the hand-written demo. No shim this time.
    let class_dir = "../04-cpp-class-shim/cpp";

    cxx_build::bridge("src/main.rs")
        .file(format!("{class_dir}/word_counter.cpp"))
        .include(class_dir)
        .include("include")
        .std("c++17")
        .compile("cxx-demo");

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed={class_dir}");
}
