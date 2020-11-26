#![allow(unused)]

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statements_vs_expressions() -> i32 {
    let x = 1; // statement
    let y = 2; // statement
    let z = x + y; // statement that encompasses the "x + y" expression
    z // expression
}

fn expression_in_block() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
