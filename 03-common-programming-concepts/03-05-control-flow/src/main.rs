#![allow(unused)]

// Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert
// non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean
// as its condition.
fn if_expressions() {
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

fn if_else_expressions() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}

fn if_let() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn loops_with_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn loops_with_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn loops_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn loops_with_for_via_rev() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn main() {
    loops_with_for_via_rev();
}
