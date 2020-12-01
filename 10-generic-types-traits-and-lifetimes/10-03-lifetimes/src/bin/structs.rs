// This lifetime annotation means an instance of "ImportantExcerpt" can't outlive
// the reference it holds in its "part" field.
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// We create an instance of the "ImportantExcerpt" struct that holds a reference to
// the first sentence of the "String" owned by the variable "novel". The data in "novel"
// exists before the "ImportantExcerpt" instance is created. In addition, "novel" doesn’t
// go out of scope until after the "ImportantExcerpt" goes out of scope, so the reference
// in the "ImportantExcerpt" instance is valid.
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("i: {:#?}", i);
}
