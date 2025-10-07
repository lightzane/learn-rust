use rand::random_range; // rand = external "crate" (dependency)
// use std::cmp::Ordering;
// use std::io;
// OR
use std::{cmp::Ordering, io}; // std = standard library

fn main() {
    let secret_number = random_range(1..=10); // OR rand::random_range(1..=10);
    let mut attempts: u32 = 0;

    const TITLE: &str = "Guess the number";
    const INSTRUCTIONS: &str = "Please input your guess.";
    let err_input_msg = "Failed to read line";

    println!("Title: {TITLE}");

    loop {
        attempts += 1;

        println!("{}", INSTRUCTIONS); // OR println!("{INSTRUCTIONS}");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect(err_input_msg);

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // .parse() returns a Result enum (Ok or Err)
        let guess: u32 = match guess.trim().parse() {
            // OR
            /* let guess = match guess.trim().parse::<u32>() { */
            Ok(num) => num,
            Err(_) => {
                // OR /* Err(_) => continue */
                // Gracefully handle the error
                println!("⚠️  Please type a number!\n");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // .cmp() function returns an Ordering enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🏆 You win!");
                break;
            }
        }

        println!();
    }

    println!();
    println!("The secret number is: {secret_number}");
    println!("You guessed it in {attempts} attempts.");
    println!();
    println!("Press Enter to exit...");

    io::stdin()
        .read_line(&mut String::new())
        .expect(err_input_msg);
}
