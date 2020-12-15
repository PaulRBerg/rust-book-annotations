#![allow(unused)]
unsafe fn dangerous() {}

pub fn run() {
    unsafe {
        dangerous();
    }
}
