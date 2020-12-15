#![allow(unused)]
use std::fmt;
use std::io::Result;

fn simple_alias() {
    // "Kilometers" is a synonym for "i32".
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

// Introducing a type alias "Thunk" to reduce repetition.
fn alias_for_long_type() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {}

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| ())
    }
}

// Type aliases are also commonly used with the "Result<T, E>" type for reducing repetition.
// This is a "Result<T, E>" with the "E" filled in as "std::io::Error".
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

pub fn run() {
    alias_for_long_type();
}
