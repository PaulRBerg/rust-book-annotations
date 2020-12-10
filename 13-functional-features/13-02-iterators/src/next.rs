#![allow(unused)]

// All iterators implement a trait named Iterator that is defined in the standard library.
// It looks like this:
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub fn iterator_that_moves() {
    let v1 = vec![1, 2, 3];
    let v1_iter_with_move = v1.into_iter();

    // Vector cannot be used after this
    for val in v1_iter_with_move {
        println!("Got: {}", val);
    }

    // This code would not compile.
    // println!("v1: {:?}", v1);
}

pub fn iterator_with_mutable_references() {
    let mut v1 = vec![1, 2, 3];
    let v1_iter_with_move = v1.iter_mut();

    // Vector cannot be used after this
    for val in v1_iter_with_move {
        println!("Got: {}", val);
    }
}

#[test]
pub fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // We need to make "v1_iter" mutable: calling the next method on an iterator changes
    // internal state that the iterator uses to keep track of where it is in the sequence.
    let mut v1_iter = v1.iter();

    // The values we get from the calls to "next()" are immutable references to the values
    // in the vector.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
