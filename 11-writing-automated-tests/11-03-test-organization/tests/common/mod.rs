// Naming this file as "common/mod.rs" instead of "common.rs" tells Rust
// not to treat the common module as an integration test file.
//
// Files in subdirectories of the tests directory don’t get compiled as
// separate crates or have sections in the test output
pub fn setup() {
    // setup code specific to your library's tests would go here
}
