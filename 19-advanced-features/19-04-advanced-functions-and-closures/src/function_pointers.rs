#![allow(unused)]

// Functions coerce to the type "fn" (with a lowercase f), not to be confused
// with the "Fn" closure trait. The "fn" type is called a function pointer.
// Unlike closures, "fn" is a type rather than a trait.
//
// Function pointers implement all three of the closure traits: "Fn", "FnMut"
// and "FnOnce", so you can always pass a function pointer as an argument
// for a function that expects a closure.

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn call_do_twice() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

fn convert_numbers_to_strings() {
    let list_of_numbers = vec![1, 2, 3];

    // Variant 1
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Variant 2, which requires the fully qualified syntax
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// Here we create "Status::Value" instances using each "u32" value in the range
// that "map" is called on by using the initializer function of "Status::Value".
fn initializers() {
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("list_of_statuses: {:#?}", list_of_statuses);
}

pub fn run() {
    initializers();
}
