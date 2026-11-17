use core::ffi::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::panic::catch_unwind;

/// Returns a newly allocated string. Free it with `sharp_string_free`.
///
/// # Safety
/// `name` must be NULL or a valid NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sharp_greet(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: non-null and NUL-terminated per the contract above.
    let name = unsafe { CStr::from_ptr(name) };
    // C strings are just bytes: they may not be UTF-8.
    let name = name.to_string_lossy();
    let greeting = format!("Hello, {name}! Greetings from Rust.");
    match CString::new(greeting) {
        Ok(s) => s.into_raw(), // ownership leaves Rust here...
        Err(_) => std::ptr::null_mut(),
    }
}

/// # Safety
/// `s` must be NULL or a pointer from `sharp_greet`, freed only once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sharp_string_free(s: *mut c_char) {
    if !s.is_null() {
        // SAFETY: came from `CString::into_raw`, per the contract above.
        drop(unsafe { CString::from_raw(s) }); // ...and comes back here.
    }
}

/// Returns 0 on success, -1 if the Rust code panicked.
///
/// # Safety
/// `out` must be a valid pointer to an `int`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sharp_divide(
    a: c_int,
    b: c_int,
    out: *mut c_int,
) -> c_int {
    // `a / b` panics when b == 0. Catch it before it reaches the boundary.
    match catch_unwind(|| a / b) {
        Ok(value) => {
            // SAFETY: `out` is valid per the contract above.
            unsafe { *out = value };
            0
        }
        Err(_) => -1,
    }
}
