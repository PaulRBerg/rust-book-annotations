#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // Methods that call "next" are called consuming adaptors.
    let total: i32 = v1_iter.sum();

    // We aren’t allowed to use v1_iter after the call to "sum", because "sum"
    // takes ownership of the iterator we call it on.
    // println!("v1_iter: {:?}", v1_iter);

    assert_eq!(total, 6);
}
