//! # Learn Rust
//!
//! A collection of examples and exercises to learn Rust.
//!
//! (This will show up in the `index.html` file when you run `cargo doc`)

/// Adds one to the given integer 👍🏻
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = learn_rust::add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Re-exports: make internal structure available at the crate root
pub use self::kinds::PrimaryColor; // can now be accessed as learn_rust::PrimaryColor
pub use self::kinds::SecondaryColor; // can now be accessed as learn_rust::SecondaryColor
pub use self::utils::mix; // can now be accessed as learn_rust::mix

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        unimplemented!()
    }
}
