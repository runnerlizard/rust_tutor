fn main() {
    let user1 = build_user(String::from("mail@mail.ru"), String::from("lizard"));
    let user2 = User {
        email: String::from("new@mail.ru"),
        ..user1
    };
    println!("{}\n{}", user1.email, user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}