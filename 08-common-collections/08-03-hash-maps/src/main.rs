#![allow(unused)]
use std::collections::HashMap;

fn create_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:#?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<String, i32> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores: {:#?}", scores);
}

// For types that implement the Copy trait, like i32, the values are copied into
// the hash map. For owned values like String, the values will be moved and the
// hash map will be the owner of those values.
fn ownership_and_hash_maps() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

fn accessing_values_in_hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score: {:?}", score.unwrap());
}

fn iterate_over_hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

// Overwrite
fn update_hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

// Only insert a value if the key has no value
fn update_hash_maps_v2() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn update_hash_maps_v3() {
    // Updating a value based on the old Value
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main() {
    update_hash_maps_v3();
}
