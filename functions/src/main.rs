fn main() {
    println!("Hello, world!");

    another_function_1();
    another_function_2(5);
    print_labeled_measurement_1(5, 'h');
    print_labeled_measurement_2(5, "test");

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");
    
    let x = five();
    println!("The value of x is {x}");

    let x = plus_one(5);
    println!("The value of x is {x}");
}

fn another_function_1() {
    println!("Another function.");
}

fn another_function_2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement_1(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn print_labeled_measurement_2(value: i32, unit_label: &str) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
