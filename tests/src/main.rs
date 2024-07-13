struct Player {
    name: String,
    level: i32,
    hp: i32,
    mp: i32,
}
impl Player {
    fn increase_level(&mut self) {
        self.level += 1;
        self.hp += 10;
        self.mp += 5;
    }
}

fn main() {
    let mut player1 = Player {
        name: String::from("Yang"),
        level: 1,
        hp: 100,
        mp: 50,
    };

    println!("Player: {}", player1.name);
    println!("Level: {}", player1.level);
    println!("HP: {}", player1.hp);
    println!("MP: {}", player1.mp);

    player1.increase_level();

    println!("Player: {}", player1.name);
    println!("Level: {}", player1.level);
    println!("HP: {}", player1.hp);
    println!("MP: {}", player1.mp);
}
