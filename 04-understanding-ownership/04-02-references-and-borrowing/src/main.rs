#![allow(unused)]

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn immutable_references() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn invalid_mutable_references() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // This line would not compile
    // let r2 = &mut s;

    println!("{}", r1);
}

fn invalid_mutable_references_2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // This line would not compile
    // let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2);
}

fn valid_mutable_references() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn valid_mutable_references_2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// fn dangling_references() {
//     let reference_to_nothing = dangle();
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn no_dangling_references() {
    let reference = no_dangle();
}

// The Rules of References
// + At any given time, you can have either one mutable reference or any number of immutable references.
// + References must always be valid.
fn main() {
    no_dangling_references();
}
