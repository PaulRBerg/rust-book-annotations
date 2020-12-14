pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// This works differently from defining a struct that uses a generic type parameter with
// trait bounds. A generic type parameter can only be substituted with one concrete type
// at a time, whereas trait objects allow for multiple concrete types to fill in for the
// trait object at runtime.
impl Screen {
    // Duck typing
    // If it walks like a duck and quacks like a duck then it must be a duck.
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        ()
    }
}
