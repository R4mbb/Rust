#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let dne: Option<&i32> = v.get(100);
    match dne {
        Some(dne) => println!("True"),
        None => println!("False"),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("{}", *i+50);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{:?}", i);
    }
}
