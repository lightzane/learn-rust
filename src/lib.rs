pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn bugged_add(left: u64, right: u64) -> u64 {
    left + right + 1
}

fn private_add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn greeting(_name: &str) -> String {
    String::from("Hello")
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(3, 5);
        assert_eq!(result, 8, "it should work 3 and 5");
    }

    #[test]
    #[should_panic] // without assertion on panic message
    fn another() {
        panic!("Make the test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Edward");
        assert!(
            result.contains("Edward"),
            "Custom failed error: Did not contain name! Value was {result}"
        )
    }

    #[test]
    // with assertion on panic message
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn failed_but_dont_panic() -> Result<(), String> {
        let result = bugged_add(2, 5);

        if result == 7 {
            Ok(())
        } else {
            Err(String::from("2 + 5 does not equal to 7!"))
        }
    }

    #[test]
    #[ignore]
    fn pure_error() -> Result<(), String> {
        Err(String::from(
            "\
                Pure error! Can't use #[should_panic] on Result<T, E>. \
                Instead, use assert!(value.is_err())
            ",
        ))
    }

    #[test]
    fn this_test_will_pass() {
        // Output of println! will not show on success
        // To include, run: cargo test -- --show-output
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        // Output of println! will not show on failure
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }

    #[test]
    fn allows_test_private() {
        // Note: IDEs won't be able to suggest auto-completion for private function
        assert_eq!(private_add(1, 1), 2);
    }
}
