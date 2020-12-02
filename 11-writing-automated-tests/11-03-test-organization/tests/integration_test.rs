// We didn't need to bring "adder" into the current scope in the unit tests.
// The reason is that each file in the tests directory is a separate crate,
// so we need to bring our library into each test crate’s scope.
use adder;

// mod common;

// We don’t need to annotate any code in the "tests" directory with "#[cfg(test)]".
#[test]
fn it_adds_two() {
    // common::setup();
    assert_eq!(4, adder::add_two(2));
}
