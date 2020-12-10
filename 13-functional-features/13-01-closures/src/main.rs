#![allow(unused)]

mod moving;

fn foo() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    // Example of a closure that refers to a variable in its enclosing scope.
    assert!(equal_to_x(y));
}

fn bar() {
    let x = 4;

    // This would not compile. It would throw an error:
    // "Can't capture dynamic environment in a fn item"
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }

    let y = 4;

    // assert!(equal_to_x(y));
}

fn baz() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

// Closures can capture values from their environment in three ways:
//
// 1. FnOnce
// 2. FnMut
// 3. Fn
//
// When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment.
fn main() {
    moving::main();
}
