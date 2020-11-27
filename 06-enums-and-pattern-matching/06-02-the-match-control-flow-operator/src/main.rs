#![allow(unused)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn call_value_in_cents() {
    let nickel = Coin::Nickel;
    let result = value_in_cents(nickel);
    println!("Result: {}", result);
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum CoinV2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_v2(coin: CoinV2) -> u8 {
    match coin {
        CoinV2::Penny => 1,
        CoinV2::Nickel => 5,
        CoinV2::Dime => 10,
        CoinV2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn call_value_in_cents_v2() {
    let quarter = CoinV2::Quarter(UsState::Alaska);
    let result = value_in_cents_v2(quarter);
    println!("Result: {}", result);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn call_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// This function would not compile
// fn non_exhaustive_plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

fn the_underscore_placeholder() {
    let some_u8_value = 7u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    };
}

fn main() {
    the_underscore_placeholder();
}
