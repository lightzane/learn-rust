#[cfg(test)] // compile the tests module only when running tests (not on build or run)
mod tests;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods must have a parameter named self of type Self for their first parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Note that we can choose to give a method the same name as one of the struct’s fields
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions that don't have self as their first parameter
    // are often used for constructors that will return a new instance of the struct.
    // In JavaScript, we might call this a static method.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("The rectangle has a nonzero width: {}", rect1.width());
    println!("The rectangle width is: {}", rect1.width);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("The square is: {:?}", sq);
    println!("The area of the square is {} square pixels.", sq.area());
}
