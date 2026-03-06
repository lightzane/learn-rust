pub fn run() {
    println!("\x1b[36;7m Lifetimes - Introduction and Generics in Functions \x1b[0m");

    /*
     * Rust have "borrow checker"
     * `x` goes out of scope here, so `r` would be a dangling reference
     *
     * lifetimes:
     * 'a = lifetime of the reference `r`
     * 'b = lifetime of the reference `x`
     *
     * let r;                    / ------------+-- 'a
     *                           /             |
     * {                         /             |
     *    let x = 5;             / -+-- 'b     |
     *    r = &x;                /  |          |
     * }                         / -+          |
     *                           /             |
     * println!("r: {}", r);     / ------------+
     *
     */

    let r;

    let x = 5;
    r = &x;

    println!("r: {}", r); // * `x` is still valid here, so `r` is not a dangling reference

    // Base lifetime annotation (see fn longest)
    let string1 = String::from("long string is long");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{result}'");

    // * Lifetime Annotation Syntax
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // * Different concrete lifetimes
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{result}'");
    }
    // 👆🏻 Above code is approved by borrow checker since
    // `string1` is valid until the end of the outer scope
}

// Just like if you have multiple return types, you need to specify the concrete type of each return value
// fn longest(x: &str, y: &str) -> &str { // signature does not say whether it is borrowed from `x` or `y`
//     if x.len() > y.len() { x } else { y }
// }

// Hence, if you have multiple lifetimes,
// you need to specify the concrete lifetime of each reference
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// fn mexample_1() {
//     let string1 = String::from("long string is long");
//     let result;

//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str()); // ❌
//     }

//     println!("The longest string is '{result}'");
//     // ! Above code is rejected by borrow checker since
//     // `string2` is dropped at the end of the inner scope,
//     // so `result` would be a dangling reference
//     // even though `string1` is still valid until the end of the outer scope

//     // Rust knows this since we specified the same lifetime `'a` for both `x` and `y`,
//     // so the return value must also have the same lifetime `'a`
// }

// If we only return 'x' then we just have to specify lifetime for 'x' and not 'y'
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// If reference return is not related to any of the parameters, it will become a dangling reference
// since it would not be valid for the entire lifetime of the function
// So it's better to return an owned value instead of a reference in this case
// And the calling function is then responsible for cleaning up the value.
//
// If reference is returned instead of an owned value, then this happens:
// fn return_reference<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str() // ❌
//     // ! Above code is rejected by borrow checker since
//     // `result` is dropped at the end of the function,
//     // so the return value would be a dangling reference
// }
fn return_owned<'a>(x: &str, y: &str) -> String {
    String::from("really long string")
}
