enum Message {
    Hello { id: i32 },
}

// Using "@" lets us test a value and save it in a variable within one pattern.
pub fn run() {
    let msg = Message::Hello { id: 5 };

    // By specifying "id_variable @" before the range "3..=7", we are capturing whatever
    // value matched the range while also testing that the value matched the range pattern.
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // The code associated with the arm doesn’t have a variable that contains the actual
        // value of the id field.
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
