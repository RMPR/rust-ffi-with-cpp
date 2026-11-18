mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use ffi::Point;

// The safe wrappers stay: bindgen generates declarations, not safety.
pub fn safe_add(a: i32, b: i32) -> i32 {
    // SAFETY: plain integers in, plain integer out.
    unsafe { ffi::add(a, b) }
}

pub fn magnitude(p: Point) -> f64 {
    // SAFETY: `Point` is #[repr(C)] and passed by value.
    unsafe { ffi::point_magnitude(p) }
}

pub fn scale(p: &mut Point, factor: f64) {
    // SAFETY: `p` is a valid, exclusive pointer for the whole call.
    unsafe { ffi::point_scale(p, factor) }
}

fn main() {
    println!("add(2, 40)   = {}", safe_add(2, 40));

    let mut p = Point { x: 3.0, y: 4.0 };
    println!("{p:?} magnitude = {}", magnitude(p));

    scale(&mut p, 2.0);
    println!("{p:?} magnitude = {}", magnitude(p));
}
