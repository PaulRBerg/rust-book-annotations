#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::mutable_rc::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// By using "RefCell<T>"", we have an outwardly immutable "List" value. But we can
// use the methods on "RefCell<T>" that provide access to its interior mutability
// so we can modify our data when we need to.
pub fn run() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
