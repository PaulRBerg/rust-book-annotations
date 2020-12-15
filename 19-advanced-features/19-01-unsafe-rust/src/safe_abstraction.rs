#![allow(unused)]
use std::slice;

// Safe abstraction over an unsafe block.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len: usize = slice.len();
    let ptr: *mut i32 = slice.as_mut_ptr();

    assert!(mid <= len);

    // Borrowing different parts of a slice is fundamentally okay because the two slices
    // aren't overlapping, but Rust isn't smart enough to know this.
    // (&mut slice[..mid], &mut slice[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn correct_split_of_slice() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // This function can't be implemented using only safe Rust.
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn invalid_split_of_slice() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    // Produces a "Segmentation fault: 11" error, because we don't own the memory at
    // this arbitrary location, and there is no guarantee that the slice this code
    // creates contains valid "i32" values.
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    println!("slice: {:?}", slice);
}

pub fn run() {
    invalid_split_of_slice();
}
