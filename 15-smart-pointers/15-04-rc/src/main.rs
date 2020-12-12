#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // We could have called "a.clone()" rather than "Rc::clone(&a)", but Rust's convention
    // is to use "Rc::clone" in this case. The implementation of "Rc::clone" doesn't make a
    // deep copy of all the data like most types' implementations of clone do.
    // By using Rc::clone for reference counting, we can visually distinguish between the
    // deep-copy kinds of clones and the kinds of clones that increase the reference count.
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
