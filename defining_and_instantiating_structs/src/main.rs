fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("struct user1: {:#?}", user1);

    let mut user2 = build_user("someone@example.com", "someusername123");
    user2.email = String::from("anotheremail@example.com");
    println!("struct user2: {:#?}", user2);

    let user3 = User {
        email: String::from("another@example.com"),
        // username: String::from("someusername123").cl,
        ..user1
    };
    println!("struct user3: {:#?}", user3);
    // println!("struct user1: {:#?}", user1);

    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like straucts without any fields
    let subject = AlwaysEqual;
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1,
    }
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like straucts without any fields
struct AlwaysEqual;
