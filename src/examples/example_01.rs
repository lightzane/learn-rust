pub fn run() {
    println!("\x1b[36;7m Generic Data Types - Example 1 \x1b[0m");

    let list = [5, 2, 3, 4, 1];
    let result1 = largest_i32(&list);
    let result2 = largest(&list);
    println!("The largest number is {result1}");
    println!("The largest number is {result2}");

    let list = vec!['y', 'm', 'a', 'q'];
    let result1 = largest_char(&list);
    let result2 = largest(&list);
    println!("The largest char is {result1}");
    println!("The largest char is {result2}");
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            // 👆🏻 This operator will cause error when `PartialOrd` is not implemented
            largest = item;
        }
    }

    largest
}
