#![allow(dead_code, unused_variables)]

struct Point<T> {
    x: T,
    y: T,
}

fn proper_types() {
    let integer = Point { x: 5, y: 10 }; // both are i32
    let float = Point { x: 1.0, y: 4.0 }; // both are f32
}

// The fields x and y must be the same type because both have the same generic data type T.
fn invalid_types() {
    // One is i32, the other is float.
    // let wont_work = Point { x: 5, y: 4.0 };
}

// To define a "Point" struct where "x" and "y" are both generics but could have different
// types, we can use multiple generic type parameters.
#[derive(Debug)]
struct PointV2<T, U> {
    x: T,
    y: U,
}

fn proper_types_v2() {
    let both_integer = PointV2 { x: 5, y: 10 };
    let both_float = PointV2 { x: 1.0, y: 4.0 };
    let integer_and_float = PointV2 { x: 5, y: 4.0 };

    println!("integer_and_float: {:#?}", integer_and_float);
}

fn main() {
    proper_types_v2();
}
