#![allow(unused)]
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*wavings arms furiously*");
    }
}

fn instantiate_persons() {
    let person = Human;

    // This code will print "*waving arms furiously*", showing that Rust called
    // the "fly" method implemented on "Human" directly.
    person.fly();

    // But we can also call the "fly" methods defined in "Pilot" and "Wizard"
    Pilot::fly(&person);
    Wizard::fly(&person);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn run() {
    // Because "Animal::baby_name" is an associated function rather than a method, and
    // thus doesn't have a self parameter, Rust can’t figure out which implementation
    // of "Animal::baby_name" we want.
    // println!("A baby dog is called a {}", Animal::baby_name());

    // Fully qualified syntax.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
