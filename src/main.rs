use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy)] // -> `Clone, Copy` was added since it is required by println! when printing `user1_pref`
enum ShirtColor {
    Gold,
    Yellow,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preferences: Option<ShirtColor>) -> ShirtColor {
        user_preferences.unwrap_or_else(|| self.most_stocked())
        /*
            We specify the closure expression `|| self.most_stocked()` as the argument to `unwrap_or_else`.
            This is a closure that takes no parameters itself
            (if the closure had parameters, they would appear between the two vertical pipes).


        */
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_gold = 0;
        let mut num_yellow = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Gold => num_gold += 1,
                ShirtColor::Yellow => num_yellow += 1,
            }
        }

        if num_gold > num_yellow {
            ShirtColor::Gold
        } else {
            ShirtColor::Yellow
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
}

fn example_1() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Gold, //
            ShirtColor::Gold,
            ShirtColor::Yellow,
        ],
    };

    let user1_pref = Some(ShirtColor::Gold);
    let user2_pref = None; // type is inferred after this passed to store.giveaway()

    let giveaway1 = store.giveaway(user1_pref);
    let giveaway2 = store.giveaway(user2_pref);

    println!("User with preference {:?} gets {:?}", user1_pref, giveaway1);
    println!("User with preference {:?} gets {:?}", user2_pref, giveaway2);
}

fn example_2() {
    #[allow(unused_variables)]
    // Optional: type annotations on `num`, we can put for explicitness
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::new(2, 0));
        num
    };

    /*
        In JavaScript,
        The syntax for an anonymous function is

            `function (parameters) { body }`
        Or

            `(parameters) => { body }` for arrow functions.

        In Rust, the syntax for a closure is

            `|parameters| { body }`.

        The parameters are enclosed in vertical bars `| |`,
        and the body is enclosed in curly braces `{ }`.
    */

    #[rustfmt::skip]
    fn  add_one_v1 (x: u8) -> u8 { x + 1  }
    let add_one_v2 = |x: u8| -> u8 { x + 1 };
    let add_one_v3 = |x| x + 1;
    // 👆🏻 Note: type inference is used here,
    // so we don't need to specify the types of `x` and the return value.
    // But if the function is not used, the compiler won't be able to infer the types,
    // and it will result in an error.

    add_one_v1(1);
    add_one_v2(1);
    add_one_v3(1);
    /*
        if `add_one_v3` is not used,
        the compiler will not be able to infer the types of `x` and the return value,
        and it will result in an error.
    */
}

// Capturing References or Moving Ownership
fn example_3() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list);
    /*
        ! 👆🏻❌ compile error
        because `borrows_mutably` is a closure that mutably borrows `list`,
        and we cannot borrow `list` as immutable while it is already borrowed as mutable.

        Mutable references can only be borrowed once at a time.
    */
    borrows_mutably();
    println!("After calling closure: {:?}", list);
    /*
        * 👆🏻✅ allowed
        because we have called `borrows_mutably()`
        which mutably borrows `list` and modifies it,
        `list` is no longer borrowed after the call to `borrows_mutably()`,
        so we can borrow `list` as immutable again to print it.
    */

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // `move` keyword is used to force the closure to take ownership of the variables it uses.
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // println!("After spawning thread: {:?}", list);
    /*
        ! 👆🏻❌ compile error
        because `list` has been moved ownership into the closure
        and is no longer available
    */
}

fn example_4() {
    #[rustfmt::skip]
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // use `sort_by_key` to order them by their width attribute from low to high.
    list.sort_by_key(|r| r.width);
    println!("Sorted by width: {list:#?}");
}
