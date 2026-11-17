use core::ffi::c_int;

#[repr(C)]
#[derive(Default)]
pub struct Stats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}

#[unsafe(no_mangle)]
pub extern "C" fn rusty_add(a: c_int, b: c_int) -> c_int {
    a + b
}

/// # Safety
/// `data` must point to `len` readable, initialized doubles.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rusty_stats(
    data: *const f64,
    len: usize,
) -> Stats {
    if data.is_null() || len == 0 {
        return Stats::default();
    }
    // SAFETY: guaranteed by the caller, see the contract above.
    let values = unsafe { std::slice::from_raw_parts(data, len) };
    Stats {
        min: values.iter().copied().fold(f64::INFINITY, f64::min),
        max: values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        mean: values.iter().sum::<f64>() / len as f64,
    }
}
