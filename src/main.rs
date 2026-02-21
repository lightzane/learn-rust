#[cfg(test)]
mod tests;

fn main() {
    #[allow(unused_mut)]
    // 👆🏻 This attribute allows us to declare a mutable variable that we won't actually mutate, preventing a compiler warning.
    let mut literal_string: &str = "Hello, world!"; // stack-allocated, immutable, and cannot grow in size.
    // ❌ This line will cause a compile-time error because string literals are immutable.
    //// literal_string.push_str(" Welcome to Rust programming.");
    println!("{}", literal_string); // -> Output: Hello, world!

    let mut string_object: String = String::from("Hello, world!"); // heap-allocated, mutable, and can grow in size.
    // ✅ This line will work because String objects are mutable and can grow in size.
    string_object.push_str(" Welcome to Rust programming.");
    println!("{}", string_object); // -> Output: Hello, world! Welcome to Rust programming.

    /*
        Heap is less organized: The memory allocator finds an empty spot
        in the heap that is big enough, marks it as being in use, and returns
        a "pointer", which is the address of that location.


        string_object = The owner / "Claim ticket"
        &string_object = The viewer (reference or borrow) / "Photo of the claim ticket"

        string_object
            - has three pieces of information stored on the Stack (fast, small memory):
                1. The pointer (address) to the heap where the actual string data is stored.
                2. The length of the string (number of characters).
                3. The capacity of the string (total allocated space in the heap).

        &string_object
            - This is a Reference (or a "Borrow")
            - It points to the same string data in the heap but does not own it.
                You can see where the luggage is.
                You can read what’s inside the bag.
                But you don't own it.

            Key Trait: You can have many people looking at the photo at once.
            When the person with the photo leaves the room, the luggage stays exactly where it is.

        Cleanup:
            string_object - when it goes out of scope, data is deleted
            &string_object - does nothing to the data when it finished
    */
    let mut try_string = String::new();
    try_string.push_str(literal_string); // ✅ Stack literal string
    // try_string.push_str(string_object); // ❌ .push_str() expects a "reference" (&str) not a String
    try_string.push_str(&string_object); // ✅ We can pass a reference ("&" or "borrow") to the String object, which is a &str.

    // In JavaScript, Arrays/Objects are stored on the heap.
    // Example:
    //
    // const obj = { name: "Alice", age: 30 };
    // const abc = obj; // This creates a reference to the same object in memory, not a copy.
    // const def = obj; // This also creates a reference to the same object in memory, not a copy.
    // obj.name = "Bob"; // This will change the name property for all references (abc and def) since they point to the same object in memory.
    //
    // console.log(abc.name); // Output: Bob
    // console.log(def.name); // Output: Bob

    // Moving Ownership:
    let s1 = String::from("🦀");
    let s2 = s1; // This moves ownership of the string from s1 to s2. s1 is no longer valid after this point.
    // println!("{}", s1); // ❌ This line will cause a compile-time error because s1 has been moved to s2 and is no longer valid.
    println!("{}", s2); // ✅ This line will work because s2 now owns the string data.

    // * Rust has a feature for using a value without transferring ownership: references.
    let s3 = &s2; // This creates a reference to the string owned by s2. s2 still owns the string, and s3 is just a reference to it.
    println!("s2: {s2}");
    println!("s3: {s3}");

    // Assigning completely new value to an existing variable:
    #[allow(unused_assignments)]
    let mut sample = String::from("🦀");
    sample = String::from("🦀🦀"); // Rust automatically calls the `drop` function to free the memory by removing the old value on the heap
    println!("{}", sample);

    // Ownership, Copy and Freedom!:
    takes_ownership(sample); // after this line, sample is no longer valid, 
    // as we move ownership, but the owner is inside the function and becomes out of scope
    // since the function did not return the ownership back to the caller, the memory is freed when the function ends.
    // println!("After takes_ownership: {sample}"); // ❌ This line will cause a compile-time error because sample has been moved to the takes_ownership function and is no longer valid.

    let x = 42;
    makes_copy(x); // x is copied into the makes_copy function, but x is still valid after this line because i32 is a Copy type.
    println!("After makes_copy: {x}"); // ✅ This line will work because x is still valid after being copied into the makes

    let new_owner = gives_ownership(); // new_owner now owns the string returned by gives_ownership
    println!("New owner: {new_owner}"); // ✅ This line will work because new
}

pub fn takes_ownership(s: String) {
    println!("Inside takes_ownership: {s}");
} // s goes out of scope here and the memory is freed

pub fn makes_copy(x: i32) {
    println!("Inside makes_copy: {x}");
} // x goes out of scope here, but nothing special happens because i32 is a Copy type

pub fn gives_ownership() -> String {
    let some_string = String::from("Hello from gives_ownership!");
    some_string // This moves ownership of the string to the caller
}
