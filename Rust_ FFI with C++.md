#### **1\. Intro & motivation (3 min)**

* Why mix Rust and C++ at all? Existing codebases, incremental migration, reusing mature libraries (OpenSSL, SQLite, game engines), and exposing Rust to other languages.  
* Set expectations: “By the end you’ll know what’s happening under the hood, and which tool to reach for.”

#### **2\. What FFI actually is (5 min)**

* FFI means calling a function compiled by one language from another.  
* The key concept is the **ABI**: calling conventions, how arguments are passed, struct layout, symbol names.  
* The **C ABI is the lingua franca**. Almost every language can speak it.  
* Why C++ is harder: name mangling, templates (no code until instantiated), overloading, exceptions, vtables, and no stable ABI. So Rust↔C++ usually means **Rust ↔ C ↔ C++**.  
* Optional: run `nm` on a compiled `.o` to show a mangled C++ symbol next to a plain C one. It’s a quick, memorable visual.

#### **3\. Rust calling C, by hand (8 min)**

* Declaring foreign functions: `unsafe extern "C" { fn add(a: c_int, b: c_int) -> c_int; }`. In the Rust 2024 edition, extern blocks must be marked `unsafe`, which is worth pointing out.  
* Types from `core::ffi`: `c_int`, `c_char`, and others. Rust’s `i32` isn’t guaranteed to be C’s `int` in principle.  
* Linking: compile the C file yourself, then tell rustc where it is with `build.rs` and `cargo:rustc-link-lib` / `rustc-link-search`. Mention the `cc` crate as a convenience, but show that it isn’t required.  
* Structs: `#[repr(C)]` and why Rust’s default layout is not C’s.  
* Why every call is `unsafe`: the compiler can’t verify the other side.  
* Write a safe wrapper function around the unsafe call. This is the idiomatic pattern.  
* **Demo:** `add` and the `Point` function.

#### **4\. C++ calling Rust, by hand (6 min)**

* Exporting: `#[unsafe(no_mangle)] pub extern "C" fn ...` (2024 edition syntax; older code uses `#[no_mangle]`).  
* `crate-type = ["staticlib"]` or `["cdylib"]` in `Cargo.toml`.  
* Hand-writing the C header, and `extern "C" { }` on the C++ side to turn off mangling.  
* Wrapping real C++ (classes, methods) behind `extern "C"` shim functions with opaque pointers. This is the general technique for “C++ through a C door.”  
* **Demo:** C++ `main` calling a Rust function.

#### **5\. The sharp edges (5 min)**

* **Ownership:** whoever allocates must free. Never `free()` Rust memory or drop C memory in Rust. Show the `CString::into_raw` / `from_raw` pair and a matching `free_string` function.  
* **Strings:** `CStr` and `CString`, NUL termination, and UTF-8 vs arbitrary bytes.  
* **Panics and exceptions:** unwinding across an `extern "C"` boundary aborts in modern Rust; use `catch_unwind` if needed. C++ exceptions must not cross into Rust either.  
* Null pointers, lifetimes the compiler can’t see, and thread safety assumptions.  
* Transition: “Writing all of this by hand is tedious and error-prone. Enter tooling.”

#### **6\. bindgen, plus a nod to cbindgen (7 min)**

* What bindgen does: reads C (and some C++) headers through libclang and generates the `extern` blocks and `#[repr(C)]` structs for you.  
* Typical setup: `build.rs` calling `bindgen::Builder`, `include!(concat!(env!("OUT_DIR"), "/bindings.rs"))`.  
* Useful knobs: allowlisting functions and types, blocklisting, opaque types.  
* Limits with C++: templates, many STL types, and inline functions don’t translate well. It’s great for C APIs and so-so for C++.  
* **Demo:** delete your hand-written declarations from section 3 and replace them with generated ones. Same program, less code.  
* One slide on **cbindgen**, the reverse direction: it generates C/C++ headers from your Rust code, replacing the hand-written header from section 4\.

#### **7\. The wider landscape (6 min)**

One slide each, or a single comparison table:

* **cxx:** a safe, opinionated bridge. You describe the boundary in a `#[cxx::bridge]` module, and it generates both sides with support for `String`, `Vec`, `unique_ptr`, and similar types.  
* **autocxx:** builds on bindgen and cxx to generate bindings from existing C++ headers more automatically.  
* **zngur:** takes the C++-side view. It exposes Rust types and APIs to C++ in an idiomatic way.  
* **crubit:** Google’s bidirectional C++/Rust interop tooling.  
* Others worth naming: **cxx-qt** for Qt, and **diplomat** for generating bindings to multiple languages from a single Rust API.  
* Mention ongoing ecosystem efforts toward better C++ interop, such as the Rust Foundation’s interop initiative.  
* Takeaway framing: they all still go through the C ABI underneath. They just automate and make safe what you did by hand in sections 3 and 4\.

#### **8\. Wrap-up & Q\&A (5 min)**

* A simple decision guide:  
  * C library → bindgen.  
  * Exposing Rust to C/C++ → cbindgen.  
  * Real two-way C++ interop → cxx or zngur.  
  * Small surface or learning → by hand.  
* Link to the demo repo.  
* Questions.

