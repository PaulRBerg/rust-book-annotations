#![allow(unused)]

fn compare_integer_and_ref() {
    let x = 5;
    // Reference pointing to the value of x
    let y = &x;

    assert_eq!(5, x);
    // Without the deference operator "*", we would get a compilation error.
    assert_eq!(5, *y);
}

fn compare_integer_and_box() {
    let x = 5;
    // Instance of a box pointing to a copied value of "x".
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
