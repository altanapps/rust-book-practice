use std::collections::HashMap;

fn main () {
    // Create a new Hashmap
    let mut scores = HashMap::new();

    // Inserting values into the Hashmap
    scores.insert(String::from("Blue"), 10);

    // Insert if it doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
}