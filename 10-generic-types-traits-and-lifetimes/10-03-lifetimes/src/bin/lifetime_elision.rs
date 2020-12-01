// The compiler applies the first rule, which specifies that each parameter gets its own lifetime.
// The compiler also applied the second rule, because there is exactly one input lifetime. The lifetime
// of the one input parameter gets assigned to the output lifetime.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {}
