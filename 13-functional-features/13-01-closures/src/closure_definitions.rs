#![allow(unused)]

fn add_one() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    // Fully annotated closure definition
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // Type annotations removed from the closure definition
    let add_one_v3 = |x| x + 1;
    add_one_v3(10);

    // Brackets removed
    let add_one_v4 = |x| x + 1;
    add_one_v4(20);
}

fn example_closure() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    // The first time we call "example_closure" with the "String" value, the compiler infers
    // the type of "x" and the return type of the closure to be "String". Those types are then
    // locked in to the closure in "example_closure", and we get a type error if we try to
    // use a different type with the same closure.
    // let n = example_closure(5);
}
