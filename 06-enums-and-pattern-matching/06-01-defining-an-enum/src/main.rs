#![allow(unused)]

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn instantiate_structs_with_enum_field() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddrLikePros {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn instantiate_enums_with_associated_values() {
    let home = IpAddrLikePros::V4(127, 0, 0, 1);

    let loopback = IpAddrLikePros::V6(String::from("::1"));
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn call_message() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn main() {
    call_message();
}
