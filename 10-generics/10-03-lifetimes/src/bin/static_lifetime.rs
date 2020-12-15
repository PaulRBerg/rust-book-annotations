fn main() {
    // Static references can live for the entire duration of the program.
    let s: &'static str = "I have a static lifetime.";
}
