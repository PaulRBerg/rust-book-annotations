pub fn map_and_collect() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // "map" is an "iterator adaptor", which allows us to change iterators
    // into different kinds of iterators.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
