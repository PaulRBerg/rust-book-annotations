mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Idiomatic
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// Non-idiomatic
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant_v2() {
    // Unclear where this function is defined
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
