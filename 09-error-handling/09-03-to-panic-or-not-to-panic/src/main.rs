use std::net::IpAddr;

fn unwrap_ip_address() {
    // We can see that 127.0.0.1 is a valid IP address, so it’s acceptable to use unwrap here.
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("home: {}", home);
}

fn main() {
    unwrap_ip_address();
}
