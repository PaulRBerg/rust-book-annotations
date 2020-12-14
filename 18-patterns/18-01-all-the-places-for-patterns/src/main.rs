#![allow(unused)]

// Formal definition of the "match" keyword:
// match VALUE {
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
// }

// Formal definition of the "let" keyword:
// let PATTERN = EXPRESSION;

// Mixing "if let", "else if", "else if let", and "else".
// The downside of using "if let" expressions is that the compiler doesn't check
// exhaustiveness, whereas with match expressions it does.
fn mashup() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // "age" is a shadowed variable that contains the value inside the "Ok" variant
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loops() {
    let v = vec!['a', 'b', 'c'];

    // "(index, value)" is the pattern taken by the for loop
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_statement() {
    let x = 5;
    // Using a pattern to destructure a tuple and create three variables at once
    let (x, y, z) = (1, 2, 3);

    // If the number of elements in the pattern doesn't match the number of elements
    // in the tuple, the overall type won't match and we'll get a compiler error.
    // let (x, y) = (1, 2, 3);
}

// The "x" is a pattern!
fn foo(x: i32) {
    // Code goes here.
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
