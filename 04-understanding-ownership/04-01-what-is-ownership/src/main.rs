#![allow(unused)]

fn string_example() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

fn scoped_strings() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
      // longer valid
}

fn move_example() {
    let x = 5;
    let y = x; // y is cloned

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved not cloned

    // This line would not compile
    // println!("{}, world!", s1);
}

fn clone_example() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn main() {
    clone_example();
}
