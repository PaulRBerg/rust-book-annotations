pub fn run() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // This is the same "y" as above.
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    // The "if" condition applies to the whole pattern "4 | 5 | 6",
    // not only to the last value "6".
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
