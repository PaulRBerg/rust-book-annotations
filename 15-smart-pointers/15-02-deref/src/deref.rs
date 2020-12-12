#![allow(unused)]

use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // The "deref" method gives the compiler the ability to take a value of any type
    // that implements "Deref" and call the "deref" method to get a "&" reference
    // that it knows how to dereference.
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn run() {
    // This works because of deref coercion. Rust turns "&MyBox<String>" into "&String"
    // by calling "deref".
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // The code we would have to write if Rust didn’t have deref coercion. The "(*m)"
    // dereferences the "MyBox<String>" into a "String". Then the "&" and the "[..]"
    // take a string slice of the "String" that is equal to the whole string to match
    // the signature of "hello".
    hello(&(*m)[..]);
}
