#![allow(unused)]

// Definition of List that uses "Box<T>"" in order to have a known size
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // By using a box, we've broken the infinite, recursive chain, so the compiler
    // can figure out the size it needs to store a "List" value.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:#?}", list);
}
