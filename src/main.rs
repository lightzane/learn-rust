use core::panic;
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

mod guess;

fn main() {
    // ****** Unrecoverable errors = `panic!` *****
    // panic!("\x1b[31;1m🔥 crash and burn! \x1b[0m");

    #[allow(unused_variables)]
    let v = vec![1, 2, 3];

    // v[99]; // ! ❌ [error] index out of bounds: the len is 3 but the index is 99

    // ***** Recoverable errors = `Result<T, E>` *****

    // * Quick Panic on Error
    let greeting_file_result = File::open("hello.txt"); // This method returns a `Result<File, Error>`
    let greeting_file = match greeting_file_result {
        Ok(f) => f, // If the file is opened successfully, we get a `File` object
        Err(e) => panic!("\x1b[31;1m🔥 Problem opening the file: {:?} \x1b[0m", e),
    };

    // * Matching on different errors
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(f) => f, // If the file is opened successfully, we get a `File` object
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("\x1b[31;1m🔥 Problem creating the file: {:?} \x1b[0m", e),
            },
            _ => {
                panic!("\x1b[31;1m🔥 Problem opening the file: {:?} \x1b[0m", e)
            }
        },
    };

    // * Alternative with `unwrap_or_else` (clean up lots of `match` nesting)
    let greeting_file = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("\x1b[31;1m🔥 Problem creating the file: {:?} \x1b[0m", err)
            })
        } else {
            panic!("\x1b[31;1m🔥 Problem opening the file: {:?} \x1b[0m", err)
        }
    });

    // * Shortcuts for Panic on error
    // The enum `Result` has the `Ok` variant, the `unwrap` will return the value inside an `Ok`
    // If not `Ok`, the `unwrap` will call `panic!`
    let greeting_file = File::open("hello.txt").unwrap();

    // * Similarly, `expect` is like `unwrap` but lets you specify the panic error message
    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");
}

// * Propagating Errors (returning the Error to the calling code instead of handling it in the function)
fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(f) => f,
        Err(e) => return Err(e), // need to explicitly write `return` since this is not the last expression in the function
    };

    let mut username = String::new();

    // implicit return since this is the last expression in the function
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// * Propagating Errors with the `?` operator (a shortcut for propagating errors)
fn read_username_from_file_short() -> Result<String, Error> {
    let mut username_file = File::open("hello.txt")?; // if fails, the error will be returned to the calling code
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // if fails, the error will be returned to the calling code
    Ok(username) // if everything is successful, return the username

    /*
        The `?` placed after a `Result` value is defined to work in almost
        the same way as the `match` expressions that we defined to handle the `Result`

        The `?` operator can only be used in functions whose
        return type is compatible with the value the ? is used on.

        The `?` operator is defined to perform an early return of a value out of the function
    */
}

// * Even shorter with method chaining
fn read_username_from_file_shorter() -> Result<String, Error> {
    let mut username = String::new();

    // if fails, the error will be returned to the calling code
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// * Shortest with `fs::read_to_string` (a convenience function that does all the work for us)
fn read_username_from_file_shortest() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

// * The `?` operator can also be used with the `Option<T>` type, in which case it will return `None` if the value is `None` and unwrap the value if it is `Some`
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()

    /*
        The `?` operator can be used in functions that return `Option<T>` as well.

        In this case, if the value is `None`,
        then the function will return `None`.

        If the value is `Some`,
        then the function will unwrap the value and continue executing the function.

        ⚠️ The `?` operator won’t automatically convert a Result to an Option or vice versa;
        in those cases, to do the conversion explicitly,
        you can use methods like the:
            - ok method on Result
            - ok_or method on Option
    */
}
