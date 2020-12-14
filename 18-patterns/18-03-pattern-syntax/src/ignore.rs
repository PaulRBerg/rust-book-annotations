#![allow(unused)]

// Completely ignore the value passed as the first argument.
// Ignoring a function parameter can be especially useful in some cases, for example,
// when implementing a trait when you need a certain type signature but the function
// body in your implementation doesn’t need one of the parameters.
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

// Using an underscore within patterns that match Some variants when we don't need
// to use the value inside the Some.
fn ignore_parts_with_underscore() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // We don't need to match on or use the values inside either Some variant, but
    // we do need to test for the case when setting_value and new_setting_value are
    // the "Some" variant.
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

// Starting a variable name with an underscore to avoid getting unused variable
// warnings.
fn ignore_unused_variables() {
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    // An unused variable starting with an underscore still binds the value, which
    // might take ownership of the value.
    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s);

    let s = Some(String::from("Hello!"));

    // However, using an underscore does not bind the value.
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn ignore_remaining_parts() {
    let origin = Point { x: 0, y: 0, z: 0 };

    // Ignoring all fields of a Point except for "x" by using ".."
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    // The syntax ".." will expand to as many values as it needs to be.
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // However, using ".." must be unambiguous. If it's unclear which values are intended
    // for matching and which should be ignored, Rust will give us an error.
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     }
    // }
}

pub fn run() {
    ignore_remaining_parts();
}
