// The following can only accept irrefutable patterns:
// + Function parameters
// + "let" statements
// + "for" loops

// The following can only accept refutable patterns:
// + "if let" expressions
// + "while let" expressions

fn main() {
    let some_option_value = Some(10);
    // Attempting to use a refutable pattern with "let" yields a compiler error.
    // let Some(x) = some_option_value;

    // But it does not yield an error when used with "if let".
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // Attempting to use an irrefutable pattern with "if let" yields a compiler warning.
    if let x = 5 {
        println!("{}", x);
    };
}
