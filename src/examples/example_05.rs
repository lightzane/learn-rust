// The Option Enum (included in Rust's prelude)

pub fn run() {
    println!("\n\x1b[36;7m Running example 05... \x1b[0m");
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<u8> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    let x: u8 = 5;
    let y: Option<u8> = Some(5);

    // let sum = x + y; // ! ❌ error: mismatched types

    if let Some(val) = y {
        let sum = x + val;
        println!("sum: {}", sum);
    }

    // * BONUS: Pattern matching with `match`
    // Using `match` expression
    // -> allows us to run code for both `Some` and `None` cases
    // -> is a control flow construct that does just this
    //    when used with enums like `Option<T>`
    // -> is more verbose than `if let` but also more powerful
    // -> every possible pattern MUST be handled in a `match` expression
    match y {
        Some(val) => {
            let sum = x + val;
            println!("sum: {}", sum);
        }
        None => println!("y is None"),
    }
}
