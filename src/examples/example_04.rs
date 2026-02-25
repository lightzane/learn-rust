enum Message {
    Quit,                       // no data associated with this variant
    Move { x: i32, y: i32 },    // has named fields like a struct
    Write(String),              // has a single string data associated with it
    ChangeColor(i32, i32, i32), // has three i32 values associated with it
}

// enum above is similar to the following struct definitions:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// ! But if we used the different structs, each of which has its
// ! own type, we couldn’t as easily define a function to take
// ! any of these kinds of messages as we could with the Message enum

// One more similarity between enums and structs:
impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Message::call() was called!");
    }
}

pub fn run() {
    println!("\n\x1b[36;7m Running example 04... \x1b[0m");

    let m = Message::Write(String::from("hello"));
    m.call();
}
