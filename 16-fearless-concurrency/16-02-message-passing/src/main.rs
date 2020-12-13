// "mpsc" stands for "multiple producer, single consumer".
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // "tx1" is the transmitting end and "rx" is the receiving end.
    let (tx1, rx1) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move || {
        let vals = vec![
            String::from("bonjour"),
            String::from("de la part de"),
            String::from("fil"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("plus"),
            String::from("messages"),
            String::from("pour"),
            String::from("toi"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Treating "rx1" as an iterator.
    for received in rx1 {
        println!("Got: {}", received);
    }
}
