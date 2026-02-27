pub fn run() {
    print!("\x1b[36;7m Example 01: Vectors \x1b[0m\n");

    let v: Vec<i32> = Vec::new();
    println!("The vector is: {:?}", v);

    // Rust conveniently provides the `vec!` macro which creates a new vector
    let v: Vec<i32> = vec![]; // empty vector with type annotation since it can't infer the type
    println!("The vector is: {:?}", v);

    let v = vec![1, 2, 3, 4, 5]; // type is inferred from the values
    println!("The vector is: {:?}", v);

    // Updating a vector
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("The updated vector is: {:?}", v);

    // 2 ways to access elements in a vector
    let third: &i32 = &v[2]; // using indexing syntax, returns a reference
    // Panics and crashes 👆🏻 if goes out of bounds (e.g. &v[10])
    println!("The third element is: {}", third);

    let third: Option<&i32> = v.get(2); // using the `get` method, returns an Option
    // Does not panic if goes out of bounds, returns None instead (e.g. v.get(10))
    match third {
        Some(value) => println!("The third element is: {}", value),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable reference to the first element
    println!("The first element is: {first}"); // No error here
    v.push(6); // ⚠️ Good since 'first' has gone out of scope; BUT...
    // println!("The first element is: {}", first); // Above line will cause an error
    /*
        Why should a reference to the first element
        care about changes at the end of the vector?

        This error is due to the way vectors work:
        Because vectors put the values next to each other in memory,
        adding a new element onto the end of the vector might
        require allocating new memory
        and copying the old elements to the new space

        In that case, the reference to the first element
        would be pointing to deallocated memory.
         The borrowing rules prevent programs from ending up in that situation.
    */

    // Iterating over the values in a vector
    let v = vec![100, 200, 300];
    for n in &v {
        println!("The value is: {n}");
    }

    // Iterating and modifying values in a vector
    let mut v = vec![100, 200, 300];
    for n in &mut v {
        *n += 50; // "*" dereference to modify the value
        println!("The modified value is: {n}");
    }
    println!("The modified vector is: {:?}", v);

    // Vector can hold values of the same type, but we can use enums to hold different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        #[allow(unused_variables)]
        let some_vector = vec![1, 2, 3, 4];

        // do stuff with some_vector
    } // <- some_vector goes out of scope and is freed here
    // println!("{:?}", some_vector); // ! ❌ error: some_vector is not found in this scope
    /*
        When the vector gets dropped, all of its contents
        are also dropped, meaning the integers it holds
        will be cleaned up.
    */

    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("Integer: {}", value),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}
