#![allow(unused)]

// This code is read as "the function <foo> returns never." Functions that
// return never are called diverging functions.
// Expressions of type "!" can be coerced into any other type. For example,
// "panic!" and "continue" have a "!" type.
fn foo() -> ! {
    panic!();
}

// The loop never ends, so "!" is the value of the expression.
fn bar() -> ! {
    print!("forever ");

    loop {
        print!("and ever ");
    }
}

pub fn run() {
    foo();
}
