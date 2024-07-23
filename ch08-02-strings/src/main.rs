fn main() {
    let data = "initial contents";
    println!("{}", data);
    let s = data.to_string();
    println!("{s}");
    let s = "initial contents".to_string();
    println!("{s}");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{s1} {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // fn add(self, s: &str) -> String {}
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");
    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{s}");

    for i in hello.chars() {
        println!("{i}");
    }
    for i in hello.bytes() {
        println!("{i}");
    }
}
