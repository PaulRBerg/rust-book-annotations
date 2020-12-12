use std::cell::RefCell;
use std::rc::{Rc, Weak};

// By specifying that the relationship from a child to its parent should be a
// "Weak<T>", we're able to have parent nodes point to child nodes and vice
// versa without creating a reference cycle and memory leaks.
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// The difference between strong and weak counts is that the latter don't need
// to be 0 for the "Rc<T>" instance to be cleaned up. Strong references are
// how you can share ownership of an "Rc<T>" instance. Weak references don't
// express an ownership relationship.
pub fn run() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // The code above doesn't create a reference cycle so it is not printing an
    // infinite output.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
