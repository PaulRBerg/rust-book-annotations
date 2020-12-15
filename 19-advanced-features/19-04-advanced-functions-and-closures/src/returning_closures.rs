#![allow(unused)]

// Error "doesn't have a size known at compile-time"
// fn returns_closure() -> dyn Fn(i32) -> i32 {}

// Solution
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

pub fn run() {}
