#![allow(unused)]

fn simple_vector() {
    // We added a type annotation here. Because we aren’t inserting any values into this vector.
    let v1: Vec<i32> = Vec::new();

    // We’ve given initial i32 values, so Rust can infer that the type of v is Vec<i32>.
    let v2 = vec![1, 2, 3];
}

fn mutate_vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector: {:?}", v);
}

fn scopes_and_vectors() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    // This line would not compile
    // println!("Vector: {:?}", v);
}

fn reading_vectors() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // Use when you want your program to crash if there’s an attempt to access an
    // element past the end of the vector
    let does_not_exist = &v[100];
    // Use this method if accessing an element beyond the range of the vector happens
    // occasionally under normal circumstances.
    let does_not_exist = v.get(100);
}

// Adding a new element onto the end of the vector might require allocating new memory
// and copying the old elements to the new space, if there isn’t enough room to put all
// the elements next to each other where the vector currently is. The reference to the
// first element would be pointing to deallocated memory
fn mutate_vector_with_references() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // This line would not compile
    // v.push(6);

    println!("The first element is: {}", first);
}

fn iterate_over_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn multi_type_vectors() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Row: {:#?}", row);
}

fn main() {
    multi_type_vectors();
}
