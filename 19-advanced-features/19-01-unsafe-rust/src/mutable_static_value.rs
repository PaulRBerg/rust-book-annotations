#![allow(unused)]

// In Rust, global variables are called static variables.
//
// Constants and immutable static variables might seem similar, but a subtle difference
// is that values in a static variable have a fixed address in memory. Constants, on the
// other hand, are allowed to duplicate their data whenever they're used.
static HELLO_WORLD: &'static str = "Bonjour, le Monde!";

fn print_hello_world() {
    println!("Global variable is: {}", HELLO_WORLD);
}

// Another difference between constants and static variables is that the latter can be
// mutable. But accessing and modifying mutable static variable is "unsafe".
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn run() {
    add_to_count(3);

    // The mere usage of mutable static variables is unsafe and requires "unsafe" blocks.
    // This is because mutable static variables can be modified by multiple threads.
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
