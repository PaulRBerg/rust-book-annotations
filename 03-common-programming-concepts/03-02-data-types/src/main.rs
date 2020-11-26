#![allow(unused)]

fn number_literals() {
    let my_decimal = 98_222; // 98,222
    let my_decimal_with_type_suffix = 98_222_u32; // still 98,222
    let my_hex = 0xff; // 255
    let my_octal = 0o77; // 63
    let my_binary = 0b1111_0000; // 240
    let my_byte = b'A'; // 65 as per ASCII
}

fn floating_points() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}

// Booleans are one byte in size
fn booleans() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

// Chars are four bytes in size and represent a Unicode Scalar Value
fn chars() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", tup.2);
}

fn arrays() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [3, 3, 3, 3, 3];
    let c: [i32; 5] = [3; 5]; // this is equivalent to array "b"

    let first = a[0];
    let second = a[1];
}

fn main() {
    arrays();
}
