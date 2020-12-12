mod custom_smart_pointer;

use custom_smart_pointer::CustomSmartPointer;

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");

    // Error: explicit destructor calls not allowed.
    // c.drop();
    // The "std::mem::drop" function is different from the drop method in the "Drop" trait.
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
