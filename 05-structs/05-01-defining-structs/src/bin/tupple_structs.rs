struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    println!("black.r: {}", black.0);
    println!("black.g: {}", black.1);
    println!("black.b: {}", black.2);

    let origin = Point(0, 0, 0);
    println!("origin.x: {}", origin.0);
    println!("origin.y: {}", origin.1);
    println!("origin.z: {}", origin.2);
}
