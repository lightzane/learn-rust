pub fn run() {
    println!("\x1b[36;7m Example 02: Strings \x1b[0m\n");

    // Creating a new empty string
    let mut s = String::new();

    let data: &str = "initial contents"; // string-literal
    let s: String = data.to_string(); // convert string-literal to String
    let s = "initial contents".to_string(); // same as above
    let s = String::from("initial contents"); // another way to create a String from a string-literal

    // Updating a String
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str() appends a string slice to a String

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str() does not take ownership of the string slice
    println!("s2 is still usable: {s2}");

    let mut s = String::from("lo");
    s.push('l'); // push() appends a single character to a String (uses single quotes for char literals)
    println!("s is now: {s}");

    // Concatenation with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3 = &s1 + &s2; // ! ❌ [compile error] cannot add `&String` to `&String`
    // * String concatenation requires an owned `String` on the left
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used; s2 is not moved because we are adding a reference to it
    // println!("s1 is no longer usable: {s1}"); // ! ❌ [compile error] s1 cannot be used because it was moved
    println!("s2 is still usable: {s2}"); // s2 can still be used because it was not moved
    println!("s3 is: {s3}");
    /*
        The string s3 will contain Hello, world!.
        The reason s1 is no longer valid after the addition,
        and the reason we used a reference to s2, has to do
        with the signature of the method that’s called when we
        use the + operator. The + operator uses the add method,
        whose signature looks something like this:

            fn add(self, s: &str) -> String {

        The compiler can coerce the &String argument into a &str, which is why we can use &s2 here.
    */

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // ⚠️ If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is: {s}");

    // 💡 For combining strings in more complicated ways, we can instead use the `format!` macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // format! does not take ownership of any of its parameters
    println!("s is: {s}");

    // # Indexing into Strings
    let s1 = String::from("hello");
    // let h = s1[0]; // ! ❌ [compile error] cannot index into a String with an integer
    /*
        This error tells a story: Rust strings don't support indexing.
        Why not? To answer this question,
        We need to understand how Rust strings are stored in memory.
    */

    // A String is a wrapper over a Vec<u8>, which means that it is a collection of bytes.
    let hello = String::from("Hola"); // 4 bytes long (properly encoded UTF-8)

    // Slicing a String (3 != З)
    let hello = String::from("Здравствуйте"); // * (note that this string begins with the capital Cyrillic letter Ze, not the number 3)
    let s = &hello[0..4]; // slicing the first 4 bytes of the string, which corresponds to the first character "З"
    println!("s is: {s}"); // Зд (not "Здра")
    /*
        This example demonstrates that slicing a string in Rust is done by byte indices, not character indices.
        Since Rust strings are UTF-8 encoded, some characters may take more than one byte.
        In this case, the first character "З" takes 2 bytes, so slicing the first 4 bytes gives us "Зд".    
    */

    // Iterating over the characters in a string
    for c in "Зд".chars() {
        println!("{c}");
    }

    // Iterating over the bytes in a string
    for b in "Зд".bytes() {
        println!("{b}");
    } // Зд = 4 bytes, but 2 characters
}
