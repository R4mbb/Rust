struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("asd123"),
        email: String::from("asd123@asd.com"),
        sign_in_count: 1,
    };
    
    let user2 = User {
        email: String::from("zxc132@zxc.com"),
        ..user1
    };

    user1.email = String::from("asdasd123@asd.com");

    println!("{}", user1.sign_in_count);
    println!("{}", user2.username);


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.1);


    let subject = AlwaysEqual;
}
/*
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
*/
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
