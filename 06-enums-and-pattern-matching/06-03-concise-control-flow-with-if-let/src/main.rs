fn main() {
    let some_u8_value = Some(0u8);

    // A lot of boilerplate code
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => println!("Not three"),
    }

    let some_u8_value = Some(3u8);

    // Nicer syntax
    if let Some(3) = some_u8_value {
        println!("Three");
    } else {
        println!("Not three");
    }
}
