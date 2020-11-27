#![allow(unused)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn instantiate_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

fn mutate_struct() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_with_field_init_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn update_struct() {
    let user1 =
        build_user_with_field_init_shorthand(String::from("someone@example.com"), String::from("someusername123"));

    // Without struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // The .. notation is the struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
}

fn main() {
    update_struct();
}
