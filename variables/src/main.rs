fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("In x : {x}");
    }

    println!("Out x : {x}");
}
