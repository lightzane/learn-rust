use super::{gives_ownership, makes_copy, takes_ownership};

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_takes_ownership() {
        let test_string = String::from("Hello, Rust!");
        let new_owner = takes_ownership(test_string);
        assert_eq!(new_owner, ()); // takes_ownership does not return anything, so we expect it to return the unit type `()`
    }

    #[test]
    fn test_gives_ownership() {
        let owned_string = gives_ownership();
        assert_eq!(owned_string, "Hello from gives_ownership!");
    }

    #[test]
    fn test_makes_copy() {
        let test_int = 100;
        makes_copy(test_int);
        // After this point, test_int is still valid because i32 is a Copy type
        assert_eq!(test_int, 100);
    }
}
