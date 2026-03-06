struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn run() {
    println!("\x1b[36;7m Generic Data Types - Example 2 \x1b[0m");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mix = Point { x: 5, y: "4.0" };

    println!("The integer point is ({}, {})", integer.x, integer.y);
    println!("The float point is ({}, {})", float.x, float.y);
    println!("The mix point is ({}, {})", mix.x, mix.y);
    println!("The x value of the integer point is {}", integer.x());
}
