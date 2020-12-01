#![allow(unused)]

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn simple_point() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

struct PointV2<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointV2<T, U> {
    fn mixup<V, W>(self, other: PointV2<V, W>) -> PointV2<T, W> {
        PointV2 { x: self.x, y: other.y }
    }
}

fn mixed_up_points() {
    let p1 = PointV2 { x: 5, y: 10.4 };
    let p2 = PointV2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn main() {
    mixed_up_points();
}
