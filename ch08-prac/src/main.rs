use std::collections::HashMap;
use simple_user_input::get_input;

fn main() {
    let l = vec![123, 432, 324, 233, 567, 765, 49, 49, 233, 233, 123];
    let mut tmp = 0;

    for _ in &l {
        tmp += 1;
    }
    println!("{:?}", &l);
    println!("{}", l[tmp/2]);

    let mut map = HashMap::new();
    for num in &l {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut tmp = Vec::new();
    for (_, value) in &map {
        tmp.push(value);
    }
    let max = *tmp.iter().max().unwrap();

    for (key, value) in &map {
        if value == max {
            println!("{}", key);
        }
    }
    
    let mo = vec!["a", "e", "i", "o", "u"];

    let s1 = String::from("apple");
    let tmp = &s1[0..1];
    let s2 = &s1[1..];

    if mo.contains(&tmp) {
        let result = format!("{s1}-hay");
        println!("{result}");
    } else {
        let result = format!("{s2}-{tmp}ay");
        println!("{result}");
    }

    let mut token = 0;
    while token == 1 {
        println!(">> ");
        let input: String = get_input("Please type something....");
        println!("{}", input);
        token += 1;
    }
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}

