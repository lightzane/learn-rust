fn main() {
    let s1 = String::from("Hello");

    // let (s2, length) = calculate_length(s1);
    // println!("The length of '{s2}' is {length}"); // -> The length of 'Hello' is 5

    let length = calculate_length(&s1);
    println!("The length of '{s1}' is {length}"); // -> The length of 'Hello' is 5

    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("{s2}"); // -> Hello, world

    // ! Restriction on Mutable References
    // ! The restriction preventing multiple mutable references
    // ! to the same data at the same time allows for mutation but in a very controlled fashion.
    // let mut s = String::from("Hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // ❌ error[E0499]: cannot borrow `s` as mutable more than once at a time

    // println!("{r1} and {r2}");

    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
        r1.push_str("🦀");
        println!("r1: {r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("r2: {r2}");

    #[allow(unused_mut)]
    let mut s = String::from("Hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1: {r1} r2: {r2}"); // no problem
    // let r3 = &mut s; // ❌ error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("r1: {r1} r2: {r2} r3: {r3}"); // Makes r3 unusable because r1 and r2 are still in scope. If we comment out the line above, this will work just fine. The rule is that if you have an immutable reference to a value, you can have as many of those as you want, but you can’t have a mutable reference to that value. This is because if you had a mutable reference, then the data could be changed while the immutable references were still using it, which would cause problems.
}

// ! The issue here is we have to return the `String` to the calling function
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// *  Instead, we can provide a reference to the String value.
// A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable
fn calculate_length(s: &String) -> usize {
    s.len()
}

// ! ❌ We’re not allowed to modify something we have a reference to.
// fn change(some_string: &String) {
//     some_string.push_str(", world");
//     // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
// }

// * Mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ! Dangling References
// ! A dangling reference is a reference that points to a value that has been dropped.
// ! Rust prevents this by ensuring that references are always valid.
// let reference_to_nothing = dangle();
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

#[allow(unused)]
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
