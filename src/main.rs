fn main() {
    let x = 5; // immutable by default
    println!("The value of x is: {x}");

    // x = 6; // cannot mutate immutable variable `x`rust-analyzer E0384
    // try `rustc --explain E0384`

    let mut y = 10;
    y = 15;
    println!("The value of y is: {y}");

    /**
     * Constant variable are always IMMUTABLE
     * Name of constant variable must be in UPPER_SNAKE_CASE
     * Type of constant variable must be annotated
     * Constant variable can only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime
     */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 10800
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let z = 5;
    let z = z + 1; // shadows previous `z`
    {
        let z = z * 2; // shadows previous `z` within this block
        println!("The value of z in the inner scope is: {z}"); // 12
    }
    println!("The value of z is: {z}"); // 6

    // We can change the type in shadowing
    let spaces = "   ";
    let spaces = spaces.len(); // shadows previous `spaces`, now it's an integer (usize type in Rust)
    println!("Number of spaces: {spaces}");

    // The difference between mutability and shadowing is that with mutability, we can only change the value but not the type of the variable
    let mut a = 5;
    a = a + 1; // we can change the value of `a`
    // a = "hello"; // cannot assign `&str` to `i32`
    println!("The value of a is: {a}");
}
