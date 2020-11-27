#![allow(unused)]

fn define_option() {
    println!("The Option enum is included in the prelude");
    println!("In addition, so are its variants Some and None");

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {}", some_number.unwrap());
    println!("some_string: {}", some_string.unwrap());
}

fn invalid_addition() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
}

fn main() {
    invalid_addition();
}
