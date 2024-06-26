use std::io;

fn main() {
    loop {
        let mut num = String::new();
        let mut temperature = String::new();

        println!("1. 섭씨->화씨 / 2. 화씨->섭씨 ");
        io::stdin()
            .read_line(&mut num)
            .expect("Error1");

        let num: u32 = match num.trim().parse() {
            Ok(tmp) => tmp,
            Err(_) => continue,
        };

        println!("온도 입력 : ");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Error2");

        let temperature: u32 = match temperature.trim().parse() {
            Ok(tmp) => tmp,
            Err(_) => continue,
        };

        if num == 1 {
            let temperature = 9/5 * temperature + 32;
            println!("화씨 : {temperature}");
        }
        else if num == 2 {
            let temperature = (temperature-32) * 5/9;
            println!("섭씨 : {temperature}");
        }
        else {
            println!("잘못된 입력.");
            break;
        }
    }

}
