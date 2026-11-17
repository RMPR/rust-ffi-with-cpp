use core::ffi::{c_double, c_int};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: c_double,
    pub y: c_double,
}

unsafe extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
    fn point_magnitude(p: Point) -> c_double;
    fn point_scale(p: *mut Point, factor: c_double);
}

// Safe wrappers: the `unsafe` stays in one small, auditable place.
pub fn safe_add(a: i32, b: i32) -> i32 {
    // SAFETY: `add` takes and returns plain integers, no pointers involved.
    unsafe { add(a, b) }
}

impl Point {
    pub fn magnitude(self) -> f64 {
        // SAFETY: `Point` is #[repr(C)] and passed by value.
        unsafe { point_magnitude(self) }
    }

    pub fn scale(&mut self, factor: f64) {
        // SAFETY: `self` is a valid, exclusive pointer for the whole call.
        unsafe { point_scale(self, factor) }
    }
}

fn main() {
    println!("add(2, 40)   = {}", safe_add(2, 40));

    let mut p = Point { x: 3.0, y: 4.0 };
    println!("{p:?} magnitude = {}", p.magnitude());

    p.scale(2.0);
    println!("{p:?} magnitude = {}", p.magnitude());
}
