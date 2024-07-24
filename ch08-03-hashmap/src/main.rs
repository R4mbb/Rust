use std::collections::HashMap;

fn main() {
    let mut score = HashMap::new();
    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    println!("{}", score.get(&team_name).copied().unwrap_or(0));
    for (key, value) in &score {
        println!("{key} : {value}");
    }

    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    map.insert(field_name, field_value);
    for (i, j) in &map {
        println!("{i} : {j}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        println!("{:?}", map);
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
