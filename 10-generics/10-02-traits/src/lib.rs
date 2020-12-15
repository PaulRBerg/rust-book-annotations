#![allow(unused)]

// Trait definitions are a way to group method signatures together to define
// a set of behaviors necessary to accomplish some purpose.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait SummaryV2 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticleV2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct TweetV2 {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl SummaryV2 for TweetV2 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl SummaryV2 for NewsArticleV2 {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// Style A, syntax sugar for Style B
pub fn notify(item: &impl SummaryV2) {
    println!("Breaking news! {}", item.summarize());
}

// Style B, equivalent to Style A
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_v3(item1: &impl Summary, item2: &impl Summary) {}

// This is different to notify_v3 in that both "item1" and "item2" must be of the
// same concrete type, whereas in the other functions the types can be different as
// long as they both implement the "Summary" trait.
pub fn notify_v4<T: Summary>(item1: &T, item2: &T) {}

use std::fmt::Display;

// "item" must implement both Display and Summary because of the "+" syntax.
// With the two trait bounds specified, the body of the function can call summarize
// and use "{}"" to format item.
pub fn notify_v5(item: &(impl Summary + Display)) {}

// The + syntax is also valid with trait bounds on generic types.
pub fn notify_v6<T: Summary + Display>(item: &T) {}
