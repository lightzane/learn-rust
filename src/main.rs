mod examples;
use examples::{example_01, example_02, example_03, example_04, example_05};

fn main() {
    // Generic Data Types
    example_01::run();
    example_02::run();

    // Traits
    example_03::run();

    // Lifetimes
    example_04::run(); // generics in functions
    example_05::run(); // generics in structs, methods
}
