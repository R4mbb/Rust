fn main() {
    let s1 = String::from("hello");
    let len = calculagte_length(&s1);
    let tmp = calculagte_length(&s1);
    println!("The length of '{s1}' is {len}.");
    println!("The length of '{s1}' is {tmp}.");

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}.");

    {
        let r1 = &mut s;
        println!("r1 : {r1}");
    }
    let r2 = &mut s;
    println!("r2 : {r2}");

    //let r1 = &s;
    //let r2 = &s;
    //let r3 = &mut s;  <-  have immutable references, should not have mutable references

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");
}

fn calculagte_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
