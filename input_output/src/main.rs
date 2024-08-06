use std::io::stdin;

fn main() {
    let mut token = 0;
    let mut depart = HashMap::new()

    while token < 5 {
        let mut buffer = Vec::new();
        stdin().read_line(&mut buffer).unwrap();
        println!("{}", buffer);

        
        for i in buffer {
            println!("{}", i);
        }

        token += 1;
    }

}
