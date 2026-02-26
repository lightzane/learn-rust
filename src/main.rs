pub mod garden;
use crate::garden::vegetable::Asparagus;

// Bringing a function into scope with `use`
use learn_rust::eat_at_restaurant; // from the crate root (src/lib.rs)

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    eat_at_restaurant();
}

/*
    Example nested path

        use std::{cmp::Ordering, io};

    Instead of

        use std::cmp::Ordering;
        use std::io;

    Another example:

        use std::io::{self, Write};

    Instead of

        use std::io;
        use std::io::Write;

    Importing with glob operator (usually used in tests modules)

        use std::collections::*;
*/
