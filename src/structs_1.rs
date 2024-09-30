struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

struct User2 {
    active: bool,
    // email: &str,
    // username: &str,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;
pub fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
    };

    user1.email = String::from("anotheremail@example.com");

    // fn build_user(email: String, username: String) -> User {
    //       User {
    //           active: true,
    //           username,
    //           email,
    //           sign_in_count: 1,
    //       }
    //   }

    let user2 = User {
        active: user1.active,
        username: user1.username,
        sign_in_count: user1.sign_in_count,
        email: String::from("another@example.com"),
    };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    // ? tuple structs without named fields

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // ? unit-like structs without fields
    let subject = AlwaysEqual;

    let user3 = User2 {
        active: true,
        // username: "someusername123",
        // email: "someone@example.com",
        sign_in_count: 1,
    };

    println!("Test");
}
