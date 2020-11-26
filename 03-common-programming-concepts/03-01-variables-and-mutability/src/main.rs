#![allow(unused)]

const MAX_POINTS: u32 = 100_000;

fn increment_mutable_x() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn print_max_points() {
    println!("MAX_POINTS: {}", MAX_POINTS);
}

fn shadow_x() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn reassign_spaces() {
    let spaces = "   ";
    println!("The spaces variable is: \"{}\"", spaces);
    let spaces = spaces.len();
    println!("The spaces variable is: \"{}\"", spaces);
}

fn main() {
    reassign_spaces();
}
