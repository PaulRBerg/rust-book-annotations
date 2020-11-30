#![allow(unused)]

use std::fs::File;
use std::io::ErrorKind;

fn read_or_panic() {
    let f = File::open("hello.txt");

    let f: File = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn read_or_create_or_panic_v1() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // Matching on different errors
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created_file) => created_file,
                Err(nested_error) => panic!("Problem creating the file: {:?}", nested_error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

// Logically equivalent to the verbose version above
fn read_or_create_or_panic_v2() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_or_create_or_panic_v3() {
    // let f = File::open("hello.txt").unwrap();

    // Using "expect" instead of "unwrap" and providing good error messages can convey
    // your intent and make tracking down the source of a panic easier.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

use std::io;
use std::io::Read;

// We don’t know what the calling code will do with the returned Result. If the calling code
// gets an Err value, it could call panic! and crash the program, use a default username, or
// look up the username from somewhere other than a file, for example. We don’t have enough
// information on what the calling code is actually trying to do, so we propagate all the
// success or error information upward for it to handle appropriately.
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// The "?" placed after a "Result" value is defined to work in almost the same way as the match
// expressions we defined to handle the "Result" values above. If the value of the "Result"
// is an "Ok", the value inside the "Ok" will get returned from this expression, and the program
// will continue. If the value is an "Err", the "Err" will be returned from the whole function as
// if we had used the "return" keyword so the error value gets propagated to the calling code.
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Chain method calls immediately after the "?"
// The functionality is again the same as in Listing 9-6 and Listing 9-7; this is just a different,
// more ergonomic way to write it.
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

use std::fs;

fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

use std::error::Error;

// The "main" function is special, and there are restrictions on what its return type must be.
// One valid return type for main is the unit type "()", and conveniently, another valid return
// type is "Result<T, E>" as shown here:
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
