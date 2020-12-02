#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

// The "expensive_test" function is listed as ignored. If we want to run only the
// ignored tests, we can use "cargo test -- --ignored".
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
