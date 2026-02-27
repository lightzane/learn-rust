use std::collections::HashMap;

pub fn run() {
    println!("\x1b[36;7m Example 03: HashMap \x1b[0m\n");

    // * Creating a new empty HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score: Option<&i32> = scores.get(&team_name);
    println!("The score for team {} is {:?}", team_name, score); // prints Some(10)

    let score: i32 = scores
        .get(&team_name) // Option<&i32>
        .copied() // converts from Option<&i32> to Option<i32>
        .unwrap_or(0); // returns the value if it exists, or 0 if it doesn't
    println!("The score for team {} is {}", team_name, score); // prints 10

    for (key, value) in &scores {
        println!("{key}: {value}");
        // the order of the key-value pairs is not guaranteed
        // to be the same as the order in which they were inserted
    }

    // * Managing Ownership in Hash Maps
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new(); // HashMap<String, String>
    map.insert(field_name, field_value);
    // `field_name` and `field_value` are moved into the hash map and can no longer be used here
    // println!("{field_name}"); // ! ❌ [compile error] cannot find value `field_name` in this scope

    let field_name = String::from("Favorite color");
    let field_value = String::from("Yellow");

    let mut map = HashMap::new(); // HashMap<&String, &String>
    map.insert(&field_name, &field_value);
    println!("{field_name}: {field_value}"); // field_name can still be used because we inserted a reference to it into the hash map

    for (k, v) in &map {
        println!("inside loop 2-- {k}: {v}");
    }

    // * Updating a HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("before-- {scores:?}");

    scores.insert(String::from("Blue"), 25);
    println!("after-- {scores:?}");

    // * Adding a Key and Value ONLY If a Key Isn’t Present
    let yellow_value = scores.entry(String::from("Yellow")).or_insert(50);
    println!("yellow_value: {yellow_value}");
    let blue_value = scores.entry(String::from("Blue")).or_insert(50);
    // println!("yellow_value: {yellow_value}"); // ! ❌ cannot borrow `yellow_value` in this line since the scores will be mutated from the line above
    println!("current blue_value: {blue_value}");

    // *blue_value = 50; // ✅ No compile error (since `scores` is not yet mutated in this line); but we want to demo the behavior of `or_insert` first before we update the value

    println!("after or_insert Yellow-- {scores:?}"); // prints {"Blue": 25, "Yellow": 50}
    // Yellow was not present, so it was inserted with the value 50
    // Blue was already present, so it was not updated and remains 25

    // ! The following line will not work
    // ? *blue_value += 50; // ❌ [compile-error] out of scope since the scores will be mutated from the line above
    // 👆🏻 If you want to make this line work, put it before line 67 where `scores` was mutated
    // println!("{blue_value}"); // ! ❌ [compile error] cannot borrow `blue_value` in this line since the scores will be mutated from the line above

    //:: Using dereference (*) to update the value through the mutable reference returned by `or_insert`
    let new_yellow = scores.entry(String::from("Yellow")).or_insert(50);
    *new_yellow += 50; // yellow_value is now 100
    println!("after update using dereference '*'-- {scores:?}");

    // * Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        let count: &mut i32 = word_counts.entry(word).or_insert(0);
        // "or_insert" returns a mutable reference to the value for the specified key.
        *count += 1;
    }

    println!("word counts: {word_counts:?}");
}
