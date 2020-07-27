struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("somwusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let user2 = build_user(user1.email, user1.username);
    println!(
        "{} {} {} {}",
        user2.email, user2.username, user2.active, user2.sign_in_count
    );
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user2
    };
    println!(
        "{} {} {} {}",
        user3.email, user3.username, user3.active, user3.sign_in_count
    );

    let _black = Color(0, 0, 0);
    let _orgin = Point(0, 0, 0);
}
