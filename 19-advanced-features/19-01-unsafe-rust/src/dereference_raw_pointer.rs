#![allow(unused)]

// Raw pointers can be immutable or mutable and are written as "*const T" and
// "*mut T", respectively. The asterisk isn't the dereference operator. It's
// part of the type name.
//
// Different from references and smart pointers, raw pointers:
// + Are allowed to ignore the borrowing rules by having both immutable and
//     mutable pointers or multiple mutable pointers to the same location
// + Aren't guaranteed to point to valid memory
// + Are allowed to be null
// + Don't implement any automatic cleanup

// We can create raw pointers in safe code.
fn simple_raw_pointers() {
    let mut num = 5;

    let r1: *const i32 = &num as *const i32;
    let r2: *mut i32 = &mut num as *mut i32;

    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);

    let address = 0x012345usize;
    let r3: *const i32 = address as *const i32;
    println!("r3: {:?}", r3);
}

// Creating a pointer does no harm; it's only when we try to access the value
// that it points at that we might end up dealing with an invalid value.
fn deference_raw_pointers() {
    let mut num = 5;

    let r1: *const i32 = &num as *const i32;
    let r2: *mut i32 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }
}

pub fn run() {
    deference_raw_pointers();
}
