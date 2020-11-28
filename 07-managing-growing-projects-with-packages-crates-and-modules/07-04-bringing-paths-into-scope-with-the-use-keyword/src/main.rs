#![allow(unused)]

use std::collections::HashMap;

fn insert_into_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Using the parent modules distinguishes the two Result types
// fmt::Result and io::Result
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}

// Alternatively
use std::io::Result as IoResult;

fn function3() -> IoResult<()> {
    Ok(())
}

// External crate
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret_number: {}", secret_number);
}
