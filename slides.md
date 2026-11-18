---
title: "Rust FFI with **C++**"
theme:
  name: catppuccin-mocha
  override:
    default:
      margin:
        percent: 4
    code:
      minimum_margin:
        percent: 1
    execution_output:
      padding:
        horizontal: 2
        vertical: 0
options:
  implicit_slide_ends: true
  incremental_lists: true
  list_item_newlines: 2
---

Why mix Rust and C++ at all?
===

* **Existing codebases**: Chromium, Firefox, ...

* **Mature libraries**: e.g. Qt

<!-- pause -->
<!-- new_lines: 2 -->

> By the end you will know **what is happening under the hood**,
> and **which tool to reach for**.

<!--
speaker_note: |
  Section 1, 3 minutes.
  Next: What FFI actually is  [bullets: Calling convention, Data layout, Symbol names]
-->

What FFI actually is
===

**F**oreign **F**unction **I**nterface: calling a function compiled by one
language from another.

<!-- new_line -->

Source language can be different, but compiled code only has to agree on the
**ABI**:

* **Calling convention**: which registers, which stack slots, who cleans up
* **Data layout**: size, alignment and field order of every struct
* **Symbol names**: what the linker should look for

<!--
speaker_note: |
  Section 2, 5 minutes.
  Next: The C ABI is the lingua franca  [diagram | bullets: Name mangling, Templates, Exceptions, No stable ABI]
-->

The C ABI is the lingua franca
===

![image:width:80%](abi-diagram.png)

<!-- new_line -->

C++ is hard because it has things that neither Rust nor the C ABI can express:

* **Name mangling** and **overloading**
* **Templates**: no code exists until someone instantiates them
* **Exceptions**: there is no such thing in C
* **No stable ABI**: it differs between compilers, and sometimes versions

<!-- pause -->

But "Rust ↔ C++" nearly always means **Rust ↔ C ↔ C++**.

<!--
speaker_note: |
  Next: Seeing it: mangled vs. unmangled symbols  [code: symbols.cpp | DEMO, Ctrl+E]
-->

Seeing it: mangled vs. unmangled symbols
===

```file
path: demos/01-abi/symbols.cpp
language: cpp
start_line: 1
end_line: 6
```

```bash +exec +validate
cd demos/01-abi
c++ -c symbols.cpp -o build/symbols.o
nm build/symbols.o | grep ' T '
```

<!--
speaker_note: |
  Ctrl+E runs it. Point at _Z3addii vs _Z3adddd, then at plain c_add.
  Next: The linker's view, decoded  [DEMO, Ctrl+E | 3 bullets]
-->

The linker's view, decoded
===

```bash +exec +validate
cd demos/01-abi
c++ -c symbols.cpp -o build/decoded.o
nm --demangle build/decoded.o | grep ' T '
```

<!-- pause -->

* The **parameter types** are baked into every C++ symbol name
* The scheme is compiler-specific: Itanium here, something else on MSVC
* Whereas `c_add` is just `c_add`. That's what Rust can use

<!--
speaker_note: |
  Next: Calling C from Rust  [code: mathlib.h | bullets: Declare, Mirror, Link]
-->

Calling C from Rust
===

Let's say we want to use the following C library:

```file
path: demos/02-rust-calls-c/c/mathlib.h
language: c
start_line: 4
end_line: 11
```

<!-- new_line -->

From the Rust side we must:

1. **Declare** the functions with matching signatures
2. **Mirror** the struct with a matching layout
3. **Link** the compiled C code into the binary

<!--
speaker_note: |
  Section 3, 8 minutes.
  Next: Declaring the foreign side  [code: mathlib.h | code: main.rs | 3 bullets]
-->

Declaring the foreign side
===

<!-- column_layout: [5, 6] -->

<!-- column: 0 -->

**C**: what the library declares

```file
path: demos/02-rust-calls-c/c/mathlib.h
language: c
start_line: 4
end_line: 11
```

<!-- column: 1 -->

**Rust**: what we write to match it

```file
path: demos/02-rust-calls-c/src/main.rs
language: rust
start_line: 3
end_line: 14
```

<!-- reset_layout -->

* `unsafe extern`: **required** in the 2024 edition, optional before
* `"C"`: is the calling convention
* `core::ffi` types, because `i32` is not *guaranteed* to be C's `int`

<!--
speaker_note: |
  Next: unsafe, and the safe wrapper  [code: main.rs]
-->

`unsafe`, and the safe wrapper
===

Every foreign call is `unsafe` because the compiler cannot see the other side.

```file
path: demos/02-rust-calls-c/src/main.rs
language: rust
start_line: 16
end_line: 32
```

The idiomatic way: a small unsafe core and a safe API around it.

<!--
speaker_note: |
  Next: Using it: ordinary Rust  [code: main.rs | DEMO, Ctrl+E | shows earlier output]
-->

Using it: ordinary Rust
===

```file
path: demos/02-rust-calls-c/src/main.rs
language: rust
start_line: 34
end_line: 42
```

```bash +exec +validate +id:byhand
cd demos/02-rust-calls-c && cc -c -fPIC c/mathlib.c -o build/mathlib.o
ar rcs build/libmathlib.a build/mathlib.o
rustc --edition 2024 src/main.rs -o build/main -L build -l static=mathlib
build/main
```

<!-- snippet_output: byhand -->

<!-- incremental_lists: false -->
<!--
| Flag                  | Meaning                                         |
|-----------------------|-------------------------------------------------|
| `cc -c -fPIC`         | Compile only, position-independent like Rust    |
| `ar rcs libmathlib.a` | A static library is just an archive of `.o`     |
| `-L build`            | Where the linker should **search**              |
| `-l static=mathlib`   | What to **link**: finds `libmathlib.a`          |
| `--edition 2024`      | `rustc` alone defaults to the 2015 edition      |
-->

<!--
speaker_note: |
  Next: Calling C++ from Rust  [code: word_counter.hpp]
-->

Calling C++ from Rust
===

Let's take the following snippet:

```file
path: demos/04-cpp-class-shim/cpp/word_counter.hpp
language: cpp
start_line: 7
end_line: 15
```

A class, `std::string`, `std::map`, an exception: nothing C can express natively.

<!--
speaker_note: |
  Next: We first have to shim it behind a C door.  [code: shim.h | 3 bullets]
-->

We first have to shim it behind a C door.
===

```file
path: demos/04-cpp-class-shim/cpp/shim.h
language: c
start_line: 11
end_line: 18
```

* The class becomes an **opaque pointer**
* Every method becomes a free function taking that pointer
* Constructor and destructor become `_new` and `_free`

<!--
speaker_note: |
  Next: Where C++ stops  [code: shim.cpp]
-->

Where C++ stops
===

```file
path: demos/04-cpp-class-shim/cpp/shim.cpp
language: cpp
start_line: 11
end_line: 19
```

Exceptions are translated into **return values** here, before the boundary.

<!--
speaker_note: |
  Next: The Rust side: an opaque handle  [code: main.rs]
-->

The Rust side: an opaque handle
===

```file
path: demos/04-cpp-class-shim/src/main.rs
language: rust
start_line: 4
end_line: 22
```

Rust never learns the size or the fields of `WordCounter`. It only holds a pointer.

<!--
speaker_note: |
  Next: Making it feel like Rust  [code: main.rs]
-->

Making it feel like Rust
===

```file
path: demos/04-cpp-class-shim/src/main.rs
language: rust
start_line: 38
end_line: 46
```

```file
path: demos/04-cpp-class-shim/src/main.rs
language: rust
start_line: 62
end_line: 67
```

`Result` instead of a `bool`, `Drop` instead of a manual `_free`.

<!--
speaker_note: |
  Next: Demo: Rust drives a C++ class  [code: main.rs]
-->

Demo: Rust drives a C++ class
===

```file
path: demos/04-cpp-class-shim/src/main.rs
language: rust
start_line: 69
end_line: 79
```

<!--
speaker_note: |
  Next: Build and run  [DEMO, Ctrl+E]
-->

Build and run
===

```bash +exec +validate
cd demos/04-cpp-class-shim
c++ -c -fPIC cpp/word_counter.cpp -o build/word_counter.o
c++ -c -fPIC cpp/shim.cpp         -o build/shim.o
ar rcs build/libwordcounter.a build/word_counter.o build/shim.o
rustc --edition 2024 src/main.rs -o build/main \
    -L build -l static=wordcounter -l stdc++
build/main
```

<!-- pause -->

<!--
speaker_note: |
  Next: The pattern: an hourglass  [diagram]
-->

The pattern: an hourglass
===

![image:width:75%](hourglass.png)

<!--
speaker_note: |
  Wide and expressive at both ends, pinched to plain C in the middle. Each feature is squeezed through the waist and rebuilt on the other side.
  Next: The edges the compiler cannot see  [bullets: Exceptions, Null pointers, Lifetimes, Aliasing, Threads]
-->

The edges the compiler cannot see
===

* **Exceptions**: a C++ exception must never unwind into Rust
* **Null pointers**: every pointer coming in can be null
* **Lifetimes**: C++ keeps your pointer for as long as it likes
* **Aliasing**: `&mut T` promises exclusivity, C++ made no such promise
* **Threads**: Rust can't tell if the C++ is thread-safe. `Send`/`Sync` become your responsibility. 

<!-- pause -->
<!-- new_lines: 2 -->

> Writing all of this by hand is tedious and error-prone.
>
> **Enters tooling.**

<!--
speaker_note: |
  Next: Remember the first example?  [code | shows earlier output]
-->

Remember the first example?
===

```bash
cd demos/02-rust-calls-c
cc -c -fPIC c/mathlib.c -o build/mathlib.o    # 1. compile the C
ar rcs build/libmathlib.a build/mathlib.o     # 2. pack a static library
rustc --edition 2024 src/main.rs -o build/main \
    -L build -l static=mathlib                # 3. compile Rust, link both
build/main
```

<!-- snippet_output: byhand -->

<!--
speaker_note: |
  Next: Demo: the same program, through cargo  [DEMO, Ctrl+E]
-->

Demo: the same program, through cargo
===

```bash +exec +validate
cd demos
cargo run --quiet -p rust-calls-c
```

<!--
speaker_note: |
  Next: You do have to write a build.rs though  [code: build.rs | 3 bullets]
-->

You do have to write a `build.rs` though
===

```file
path: demos/02-rust-calls-c/build.rs
language: rust
start_line: 4
end_line: 15
```

After that we no longer type, or forget:

* The compile and archive steps, **re-run only when the C changes**
* `-L` and `-l`, passed to `rustc` for us
* The edition, the output paths, debug vs release

<!--
speaker_note: |
  Next: But you can also use bindgen  [5 bullets]
-->

But you can also use bindgen
===

Reads C (and some C++) headers through **libclang**, and writes the `extern`
blocks and `#[repr(C)]` structs for you.

<!-- new_line -->

* It is a **crate**: extra convenient to use Cargo
* Runs from `build.rs`, or as a CLI: `bindgen mathlib.h -o bindings.rs`
* Output lands in `OUT_DIR`, and can be pulled in with `include!`
* **Allowlist** the functions and types you want
* **Blocklist** what you do not, mark awkward types **opaque**

<!--
speaker_note: |
  Section 6, 7 minutes.
  Next: bindgen in build.rs  [code: build.rs]
-->

bindgen in `build.rs`
===

```file
path: demos/06-bindgen/build.rs
language: rust
start_line: 3
end_line: 21
```

<!--
speaker_note: |
  Next: bindgen's output: the struct, with layout checks  [DEMO, Ctrl+E]
-->

bindgen's output: the struct, with layout checks
===

```bash +exec +validate
cd demos
cargo build --quiet -p bindgen-demo
06-bindgen/show-bindings.sh struct
```

<!--
speaker_note: |
  Next: bindgen's output: the functions  [DEMO, Ctrl+E]
-->

bindgen's output: the functions
===

```bash +exec +validate
cd demos
cargo build --quiet -p bindgen-demo
06-bindgen/show-bindings.sh functions
```

The same declarations as our hand-written ones.

<!--
speaker_note: |
  Next: Demo: same program, less code  [code: main.rs | DEMO, Ctrl+E]
-->

Demo: same program, less code
===

```file
path: demos/06-bindgen/src/main.rs
language: rust
start_line: 1
end_line: 3
```

```bash +exec +validate
cd demos
cargo run --quiet -p bindgen-demo
```

<!--
speaker_note: |
  Next: bindgen and C++  [bullets: Templates, STL types, Inline functions, Exceptions, move semantics, destructors]
-->

bindgen and C++
===

Bindgen is great for C APIs, but not so great for C++:

* **Templates**: only concrete instantiations, and often only as opaque blobs
* **STL types**: `std::string`, `std::vector` have no stable layout to bind
* **Inline functions**: there is no symbol to link against
* **Exceptions, move semantics, destructors**: not modelled at all

<!-- pause -->
<!-- new_line -->

If the header is C++-heavy, just use a C++-aware tool.

<!--
speaker_note: |
  Section 7, 6 minutes.
  Next: cxx: WordCounter  [code: main.rs | code: factory.h]
-->

cxx: `WordCounter`
===

```file
path: demos/08-cxx/src/main.rs
language: rust
start_line: 1
end_line: 13
```

Only C++ glue left: the constructor

```file
path: demos/08-cxx/include/factory.h
language: cpp
start_line: 8
end_line: 10
```

<!--
speaker_note: |
  Next: Using it from Rust  [code: main.rs]
-->

Using it from Rust
===

```file
path: demos/08-cxx/src/main.rs
language: rust
start_line: 15
end_line: 30
```

No `unsafe`, no `CString`, no `Drop`. The C++ exception arrives as an `Err`.

<!--
speaker_note: |
  Next: Demo: same program but less boilerplate  [DEMO, Ctrl+E]
-->

Demo: same program but less boilerplate
===

```bash +exec +validate
cd demos
cargo run --quiet -p cxx-demo
```

<!--
speaker_note: |
  Next: It is still the C ABI underneath  [DEMO, Ctrl+E]
-->

It is still the C ABI underneath
===

```bash +exec +validate
cd demos
cargo build --quiet -p cxx-demo
nm target/debug/cxx-demo | grep -E ' t cxxbridge1.*(WordCounter|word_c)'
```

<!-- pause -->

Those sure look a tad bit like the `extern "C"` shims we wrote by hand.

These tools can automate a good amount of the grunt work.

<!--
speaker_note: |
  Next: The wider landscape  [table]
-->

The wider landscape
===

| Tool         | What it is                       |
|--------------|----------------------------------|
| **cxx**      | Safe two-way bridge              |
| **autocxx** (archived)  | bindgen + cxx, from C++ headers  |
| **zngur**    | Rust APIs, idiomatic in C++      |
| **crubit**   | Google's two-way interop         |
| **cxx-qt**   | cxx for Qt                       |
| **diplomat** | One Rust API, many languages     |

<!-- new_line -->

Also moving: the Rust Foundation's **C++ interop initiative**

<https://rustfoundation.org/interop-initiative/>

<!--
speaker_note: |
  Next: Which tool do I reach for?  [table | 2 bullets]
-->

Which tool do I reach for?
===

<!-- incremental_lists: false -->

| You have...                            | Reach for                     |
|----------------------------------------|-------------------------------|
| A **C library** to call from Rust      | **bindgen**                   |
| Real **two-way C++** interop           | **cxx** or **zngur**          |
| A small surface, or a desire to suffer | **nothing**: write it by hand |

<!-- new_lines: 2 -->

Whatever tool you pick, you now know what happens under the hood:

* Symbols declaration, layout matching, linking, ...
* Ownership decision, and panic/exception boundaries

<!--
speaker_note: |
  Section 8, 5 minutes including questions.
  Next: Questions?
-->

Questions?
===

<!-- jump_to_middle -->
<!-- alignment: center -->

Every snippet in this talk ran live from the `demos/` directory.

Slides and demos: <https://github.com/RMPR/rust-ffi-with-cpp>

<!--
speaker_note: |
  Next: (end of talk)
-->
