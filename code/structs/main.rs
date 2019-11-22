// in order to create new types, use structs and enums

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, Structs!");

    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.username);

    // custom email and username
    // use the rest of the values from user1
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{}", user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);

    // structs can store references to data owned by others
    // this requires 'lifetimes' in order to keep structs and references valid
}
