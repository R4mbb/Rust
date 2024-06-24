fn main() {
    let c = 'z';
    let z: char = 'ℤ';
    let cc = '😻';
    println!("one : {c}");
    println!("two : {z}");
    println!("three : {cc}");


    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("{x} {y} {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred} {six_point_four} {one}");

    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("{}", months[0]);

    let a = [3; 5];
    println!("{}", a[0]);
}
