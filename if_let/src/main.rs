enum Coin {
    AA,
    BB,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    AAA,
    BBB,
}

fn value_in_cents(coin: Coin) -> i32 {
    let mut count = 0;
    /*
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }*/
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        1
    } else {
        count += 1;
        2
    }
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    
    let a = value_in_cents(Coin::Quarter(UsState::AAA));
    println!("{}", a);
}
