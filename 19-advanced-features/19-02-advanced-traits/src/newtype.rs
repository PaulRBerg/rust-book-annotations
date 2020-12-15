use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // "Wrapper" is a tuple struct and "Vec<T>"" is the item at index 0 in the tuple.
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn run() {
    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);
    println!("w = {}", w);
}
