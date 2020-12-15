#![allow(unused)]

// Call "C" code from Rust.
extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_abs() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Let Rust code be called from "C".
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

pub fn run() {}
