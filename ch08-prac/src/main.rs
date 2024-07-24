use std::collections::HashMap;

fn main() {
    let l = vec![123, 432, 324, 233, 567, 765, 49, 49, 233, 233, 123];
    let mut tmp = 0;

    for _ in &l {
        tmp += 1;
    }
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
    println!("{:?}", map);
    println!("{:?}", tmp);
    let max = *tmp.iter().max().unwrap();

    for (key, value) in &map {
        if value == max {
            println!("{}", key);
        }
    }
    
    let mo = vec!["a", "e", "i", "o", "u"];

    let s1 = String::from("test");
    let tmp = &s1[0..1];
    let s2 = &s1[1..];

    if mo.contains(&tmp) {
        let result = format!("{s1}-hay");
        println!("{result}");
    } else {
        let result = format!("{s2}-{tmp}ay");
        println!("{result}");
    }
}
