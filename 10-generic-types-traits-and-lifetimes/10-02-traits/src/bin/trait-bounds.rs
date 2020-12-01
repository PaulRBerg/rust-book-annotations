#![allow(unused)]
use std::clone::Clone;
use std::fmt::{Debug, Display};
use traits::{self, SummaryV2, TweetV2};

// Instead of writing this ...
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// We can use a "where" clause
fn some_function_v2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

fn main() {
    println!("Hello, world!");
}

// By using impl Summary for the return type, we specify that the "returns_summarizable"
// function returns some type that implements the "SummaryV2" trait without naming the concrete type.
fn returns_summarizable() -> impl SummaryV2 {
    TweetV2 {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
