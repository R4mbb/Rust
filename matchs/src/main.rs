enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents1(coin: Coin1) -> i8 {
    match coin {
        Coin1::Penny => {
            println!("Test1");
            1
        }
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter => 25,
    }
}
fn value_in_cents2(coin: Coin2) -> i8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("Test {:?}", state);
            25
        }
    }
}


fn main() {
    let a = value_in_cents1(Coin1::Penny);
    println!("{}", a);

    let b = value_in_cents2(Coin2::Quarter(UsState::Alabama));
    println!("{}", b);
}
