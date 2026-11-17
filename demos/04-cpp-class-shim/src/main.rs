use core::ffi::{c_char, c_int};
use std::ffi::CString;

/// Opaque handle: zero-sized, cannot be constructed or moved by Rust code.
#[repr(C)]
pub struct RawWordCounter {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn word_counter_new() -> *mut RawWordCounter;
    fn word_counter_free(counter: *mut RawWordCounter);
    fn word_counter_add(
        counter: *mut RawWordCounter,
        word: *const c_char,
    ) -> bool;
    fn word_counter_count(
        counter: *const RawWordCounter,
        word: *const c_char,
    ) -> c_int;
    fn word_counter_distinct(counter: *const RawWordCounter) -> usize;
}

/// Safe, idiomatic face of the C++ class.
pub struct WordCounter {
    raw: *mut RawWordCounter,
}

impl WordCounter {
    pub fn new() -> Self {
        // SAFETY: no preconditions; C++ `new` returns a valid pointer or
        // throws, and it cannot realistically throw here.
        let raw = unsafe { word_counter_new() };
        assert!(!raw.is_null());
        Self { raw }
    }

    pub fn add(&mut self, word: &str) -> Result<(), String> {
        let c_word = CString::new(word).map_err(|e| e.to_string())?;
        // SAFETY: `raw` is valid until drop, `c_word` is NUL-terminated.
        if unsafe { word_counter_add(self.raw, c_word.as_ptr()) } {
            Ok(())
        } else {
            Err(format!("C++ rejected {word:?}"))
        }
    }

    pub fn count(&self, word: &str) -> i32 {
        let Ok(c_word) = CString::new(word) else {
            return 0;
        };
        // SAFETY: as above.
        unsafe { word_counter_count(self.raw, c_word.as_ptr()) }
    }

    pub fn distinct(&self) -> usize {
        // SAFETY: `raw` is valid until drop.
        unsafe { word_counter_distinct(self.raw) }
    }
}

impl Drop for WordCounter {
    fn drop(&mut self) {
        // SAFETY: allocated by `word_counter_new`, freed once, by C++.
        unsafe { word_counter_free(self.raw) }
    }
}

fn main() {
    let mut counter = WordCounter::new();
    for word in
        "the quick brown fox jumps over the lazy dog the end".split(' ')
    {
        counter.add(word).unwrap();
    }
    println!("'the' appears {} times", counter.count("the"));
    println!("{} distinct words", counter.distinct());
    println!("adding \"\": {:?}", counter.add(""));
} // <- Drop runs here: C++ `delete` is called for us
