use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// "Pair<T>"" only implements the "cmp_display" method if its inner type "T" implements the
// "PartialOrd" trait that enables comparison and the "Display" trait that enables printing.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// This is called a "blanket implementation".
// Note that we aren’t implementing an "unknown". While the "impl" is for generic type "T",
// the type is still bound to implement another trait, "<T: Display>".
impl<T: Display> ToString for T {
    // --snip--
}

fn main() {
    let foo = 3.to_string();
}
