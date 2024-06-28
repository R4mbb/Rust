fn main() {
    let mut s = String::from("hello123 123");
    let word = first_word(&s);
    let word2 = second_word(&s);
    //s.clear();

    println!("{word}");
    println!("{word2}");

    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..11];
    println!("{hello}, {world}");

    let len = s.len();
    let slice = &s[3..len];
    println!("{slice}");
    let slice = &s[3..];
    println!("{slice}");

    let slice = &s[0..len];
    println!("{slice}");
    let slice = &s[..];
    println!("{slice}");
    
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }

    &s[..]
}
