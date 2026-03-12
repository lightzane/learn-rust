// * Example uses internal structure ----------------------------------
// use learn_rust::kinds::PrimaryColor;
// use learn_rust::utils::mix;

// * Example uses re-exports ----------------------------------
use learn_rust::{PrimaryColor, mix}; // this works because of `pub use` in src/lib.rs

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
