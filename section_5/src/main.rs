fn main() {
    let email = String::from("email@email.com");
    let username = String::from("User Name");
    let user = build_user(email, username);

    let black = Color(0, 0, 0);
    
    println!("{:?}", user.email);
}

fn build_user(email: String, username: String) -> User {
    let user1 = User {
        username,
        email,
        active: true,
        sign_in_count: 15
    };

    return User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);