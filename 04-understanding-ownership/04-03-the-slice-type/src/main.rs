#![allow(unused)]

fn naive_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn call_naive_first_word() {
    let mut s = String::from("hello world");

    let first_word_index = naive_first_word(&s); // word will get the value 5

    println!("First word index: {}", first_word_index);

    s.clear(); // this empties the String, making it equal to ""

    // first_word_index still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. first_word_index is now totally invalid!
}

fn slices_example() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("slice: {}", slice);
    let slice = &s[..2];
    println!("slice: {}", slice);

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    println!("slice: {}", slice);
    let slice = &s[3..];
    println!("slice: {}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn call_first_word() {
    let mut s = String::from("hello world");

    let first_word = first_word(&s);

    // This line would not compile
    // s.clear(); // error!

    println!("the first word is: {}", first_word);
}

fn string_literals() {
    // The type of "foo" here is "&str". It’s a slice pointing to that specific point of the binary.
    let foo = "Hello, world!";
    let bar: &str = "Hello, world!";
}

// A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it
// allows us to use the same function on both &String values and &str values.
fn first_word_like_pros(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn call_first_word_like_pros() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word_like_pros(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word_like_pros(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_like_pros(my_string_literal);
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("Slice: {:?}", slice);
}

fn main() {
    other_slices();
}
