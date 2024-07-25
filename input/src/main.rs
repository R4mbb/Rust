use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    println!("{}", buffer);

    
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    println!("{}", n);
}

