#![allow(unused)]

// This compiles because we are always returning the "x" parameter.
// We’ve specified a lifetime parameter "'a" for the parameter "x" and the
// return type, but not for the parameter "y", because the lifetime of "y"
// does not have any relationship with the lifetime of "x" or the return value.
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// This does not compile because we are attempting to return a dangling reference.

// fn longest_v2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn main() {
    println!("Hello, world!");
}
