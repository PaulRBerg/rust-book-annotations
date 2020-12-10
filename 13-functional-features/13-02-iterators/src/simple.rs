#![allow(unused)]

// In Rust, iterators are lazy, meaning they have no effect until you call methods
// that consume the iterator to use it up. The code below doesn't do anything
// anything useful.
fn useless_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("v1_iter: {:?}", v1_iter);
}

fn working_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // The loop takes ownership of "v1_iter" and makes it mutable behind the scenes.
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn main() {
    working_iterator();
}
