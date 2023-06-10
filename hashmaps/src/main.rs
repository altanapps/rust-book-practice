use std::collections::HashMap;
use rand;


// Using a Hashmap to count the number of times a word appears in some text
fn count_words(text: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    map
}

// Find the median given a vector of numbers
fn get_median(numbers: &Vec<i32>) -> f32 {
    // Clone the vector and sort it
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    // Calculate the length of the vector
    let length = sorted_numbers.len();

    // If the length is even, return the average of the middle two numbers
    if length % 2 == 0 {
        let middle = length / 2;
        let middle_two = sorted_numbers[middle] + sorted_numbers[middle - 1];
        return middle_two as f32 / 2.0;
    } else {
        // If the length is odd, return the middle number
        let middle = length / 2;
        return sorted_numbers[middle] as f32;
    }
 }

 // Find the mode given a vector of numbers
 fn get_mode(numbers: &Vec<i32>) -> f32 {
    // Create a Hashmap to store the number of times a number appears
    let mut map = HashMap::new();

    // Iterate through the vector
    for number in numbers {
        // Insert the number into the Hashmap
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    // Find the number that appears the most
    let mut mode = 0;
    let mut max_count = 0;
    for (number, count) in &map {
        if *count > max_count {
            mode = **number;
            max_count = *count;
        }
    }
    mode as f32
}


fn main () {
    // Create a new Hashmap
    let mut scores = HashMap::new();

    // Inserting values into the Hashmap
    scores.insert(String::from("Blue"), 10);

    // Insert if it doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);

    // Define a String to count
    let text = "hello world wonderful world";

    // Count the words in the String
    let map = count_words(text);

    // Print the Hashmap
    println!("{:?}", map);

    // Print the text 
    // Available here since it wasn't borrowed 
    println!("{}", text);

    // Create a list of vectors with 20 random numbers
    let mut numbers = Vec::new();
    for _ in 0..20 {
        numbers.push(rand::random::<i32>() % 10);
    }

    // Print the vector
    println!("{:?}", numbers);

    // Get the median and return
    let median = get_median(&numbers);
    println!("Median: {}", median);

    // Get the mode and return
    let mode = get_mode(&numbers);
    println!("Mode: {}", mode);
}