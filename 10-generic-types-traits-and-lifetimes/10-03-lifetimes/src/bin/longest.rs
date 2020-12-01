#![allow(unused)]

// The lifetime of the reference returned by this function is the same as the
// smaller of the lifetimes of the references passed in. These constraints are
// what we want Rust to enforce. Remember, when we specify the lifetime parameters
// in this function signature, we are NOT changing the lifetimes of any values
// passed in or returned.
fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn correct_call() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn correct_call_v2() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// The commented function can't be compiled. Th error would show that for result to be
// valid for the "println!" statement, "string2" would need to be valid until the end
// of the outer scope.

// fn invalid_call() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

fn main() {}
