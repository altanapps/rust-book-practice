
// You could also pass a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // 1

    for (i, &item) in bytes.iter().enumerate() { // 2
        if item == b' ' { // 3
            return &s[0..i]; // 4
        }
    }

    &s[..] // 5
}

fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // 6

    println!("The first word is: {}", word);


    // &str allows us to use string slices
    let word = first_word(&s[..5]); 

    println!("The first word is: {}", word);
}