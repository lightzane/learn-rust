fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("newemail@example.com");

    let user2 = build_user(String::from("john@email.com"), String::from("john123"));
    println!("Username: {}", user2.username);

    let user3 = User {
        email: String::from("zerg@starcraft.com"),
        ..user2 // This syntax means "use the remaining fields from user2"
                // "moves" the values of the fields from user2,
                // so user2 can no longer be used after this point
    };

    println!("Email 2: {}", user2.email); // valid since it was not moved
    println!("SignIn Count 2: {}", user2.sign_in_count); // valid since this is stack-allocated data that implements the Copy trait
    println!("Active 2: {}", user2.active); // valid since this is stack-allocated data that implements the Copy trait
    // println!("Username 2: {}", user2.username); // ❌ invalid since this field was moved to user3, and String does not implement the Copy trait
    println!("Username: {}, Email: {}", user3.username, user3.email);

    let black = Color(0, 0, 0);
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2); // outputs -> Black color: (0, 0, 0)

    // Unit-like structs can be useful when you need to implement
    // a trait on some type but don’t have any data that you want
    // to store in the type itself
    #[allow(unused_variables)]
    let subject = AlwaysEqual;
}

// Struct with named fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct User {
//     active: bool,
//     username: &str, // ❌ missing lifetime specifier
//     email: &str, // ❌ missing lifetime specifier
//     sign_in_count: u64,
// }
// -> Learn more about `Lifetime`

// Tuple Struct
struct Color(i32, i32, i32);

// Unit-like Struct
struct AlwaysEqual; // behaves similarly to `()` 

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
