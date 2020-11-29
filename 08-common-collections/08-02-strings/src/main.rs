#![allow(unused)]

fn create_string() {
    let mut s = String::new();

    // Style A
    let data = "initial contents";
    let s = data.to_string();

    // Style A also works on a literal directly:
    let s = "initial contents".to_string();

    // Style B, which is equivalent to Style A
    let s = String::from("initial contents");
}

fn utf8_examples() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn update_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s: {}", s);
}

fn concatenate_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // We can do this because the compiler can coerce the &String argument into a &str
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // the + operator does take ownership of s1

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // format1 does not take ownership of s1
}

// It takes 24 bytes to encode "Здравствуйте" in UTF-8, because each Unicode scalar value
// in that string takes 2 bytes of storage. Therefore, an index into the string’s bytes
// will not always correlate to a valid Unicode scalar value.
fn index_into_strings() {
    let s1 = String::from("hello");
    // let h = s1[0];

    let hello = "Здравствуйте";
    // let answer = &hello[0];
}

fn internal_representation() {
    // The Hindi word “नमस्ते” from the Devanagari script stored as a vector of u8 values.
    let a: Vec<u8> = vec![
        224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
    ];
    // The fourth and sixth are not letters, they’re diacritics that don’t make sense on their own.
    let b: Vec<char> = vec!['न', 'म', 'स', '्', 'त', 'े'];
    // Grapheme clusters, a.k.a. what a person would call the four letters that make up the Hindi word.
    let c: Vec<&str> = vec!["न", "म", "स्", "ते"];
}

fn slice_strings() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("s: {}", s); // Зд
}

fn iterate_over_scripts() {
    let navaste: &str = "नमस्ते";
    for c in navaste.chars() {
        println!("{}", c);
    }

    println!("\n");

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn main() {
    iterate_over_scripts();
}
