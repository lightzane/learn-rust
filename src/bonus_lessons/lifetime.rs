pub fn run_demo() {
    // Lifetimes allow us to give the compiler enough information
    // about borrowed values so that it can ensure that references
    // will be valid in more situations than it could without our help.
    // Reference: https://doc.rust-lang.org/book/ch10-00-generics.html#generic-types-traits-and-lifetimes#:~:text=Lifetimes%20allow,our%20help

    let r;

    {
        let x = 5;

        // borrowing x and assigning the reference to r
        r = &x; // ❌ Thie line will cause a compile-time error `x` does not live long enough
        // At this point, r is a reference to x, and x is still valid because it is in scope.
        // However, once we exit this inner scope, x will go out of scope and be dropped,
        // which means that r will become a dangling reference if we try to use it outside
        // of this scope.
    }

    println!("r: {r}"); // ❌ This line will cause a compile-time error because r is a reference to x, which has gone out of scope and been dropped, making r a dangling reference.
}
