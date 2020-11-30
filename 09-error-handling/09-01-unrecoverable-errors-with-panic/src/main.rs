#![allow(unused)]

fn direct_panic() {
    panic!("Crash and burn");
}

fn indirect_panic() {
    let v = vec![1, 2, 3];

    v[99];
}

fn main() {
    indirect_panic();
}
