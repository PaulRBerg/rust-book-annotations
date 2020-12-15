#![allow(unused)]

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // The third lifetime elision rule applies here. There are two input lifetimes, so Rust
    // applies the first lifetime elision rule and gives both "&self" and "announcement"
    // their own lifetimes. Then, because one of the parameters is "&self", the return
    // type gets the lifetime of "&self", and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {}
