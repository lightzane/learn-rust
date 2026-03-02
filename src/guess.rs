/*
    When you have a user input to parse
    and you want to ensure that the value is within a certain range,
    you can create a custom type to represent that value
    and enforce the constraints in the constructor.

    This way, you can prevent invalid values from being created and used in your program.
    Imagine you have multiple functions, you don't want to repeat the same validation logic in each function.

    Instead, we can make a new type in a dedicated module
    and put the validations in a function to create an instance of the type
    rather than repeating the validations everywhere.

    That way, it’s safe for functions to use the new type in their signatures
    and confidently use the values they receive.
*/

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
