#![allow(unused)]

pub trait Iterator {
    // This is a placeholder type. The implementors of this trait will provide a
    // concrete type later.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

// There can only be one "impl Iterator for Counter".
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// With associated types, we can't implement a trait on a type multiple times. With
// generics, we can.
pub trait IteratorWithGenerics<T> {
    fn next(&mut self) -> Option<T>;
}
