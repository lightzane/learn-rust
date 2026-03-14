use std::{cell::RefCell, rc::Rc};

mod examples;

// ! ❌ Error ----- recursive type `List` has infinite size
// enum List {
//     Cons(i32, List),
//     Nil,
// }
/*
    Example usage:
        let list = List::Cons(1, Cons(2, Cons(3, Nil)));
*/

#[allow(dead_code)]
// * ✅ Getting a recursive type with a known size
// Box<T> is a pointer, Rust always knows how much space a pointer needs.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

struct CustomSmartPointer {
    data: String,
}

// Drop is included in the "prelude" of Rust, so we don't need to bring it into scope. (e.g. use std::ops::Drop;)
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // * ----------------------------------------------------------------------------------------------------
    // * Storing the data on the heap
    let b = Box::new(5);
    println!("b = {b}");

    // * ----------------------------------------------------------------------------------------------------
    // * Recursive data types
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // * ----------------------------------------------------------------------------------------------------
    // * Following the Reference to the Value
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, x);
    // assert_eq!(5, y); // ! ❌ can't compare `{integer}` with `&{integer}`
    assert_eq!(5, *y); // * ✅ Using the dereference operator "*" to get the value that y is referencing
    println!("The value of y is: {y}"); // But this works fine since `println!` it points to.

    // * ----------------------------------------------------------------------------------------------------
    // * Using `Box<T>` as a Reference
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, y); // ! ❌ can't compare `{integer}` with `Box<{integer}>`
    assert_eq!(5, *y); // * ✅ Using the dereference operator "*" to get the value that y is referencing
    println!("The value of y is: {y}"); // But this works fine since `println!` it points to.

    // * ----------------------------------------------------------------------------------------------------
    // * Running code cleanup (must implement Drop - see CustomSmartPointer struct)
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    {
        let _e = CustomSmartPointer {
            data: String::from("immediate drop since i am inside this scope"),
        };
    }

    println!("CustomSmartPointers created.");

    let saved_data = c.data.to_string();

    // std::mem::drop is also included in prelude, so we don't need to bring it into scope. (e.g. use std::mem::drop;)
    // it is different from the Drop trait's drop method
    drop(c); // force-drop

    println!(
        "CustomSmartPointer with saved data \"{}\" dropped before the end of main.",
        saved_data
    );

    let e = d; // transferring ownership (does not call the drop)

    println!("The 'd' was tranferred to 'e': {}", e.data);

    // * ----------------------------------------------------------------------------------------------------
    // * Enabling multiple ownership with Rc<T> ('reference counting' Smart Pointer)
    // * Rc<T> keeps track of the number of references
    // ⚠️ Only used in single-threaded scenarios
    // ⚠️ The value is immutable when using Rc<T>
    // let a = String::from("value");
    // let b = a; // moving/transfering of ownership
    // let c = a; // ! ❌ [Error] value borrowed here after move ('a' doesn't own the value anymore - hence, 'a' got nothing)
    let a = Rc::new(String::from("value"));
    let _b = Rc::clone(&a);
    let _c = Rc::clone(&a); // * ✅ `Rc::clone()` doesn't actually clone the data, it just increments the reference count
    println!(
        "Reference count after creating a, b, c: {}",
        Rc::strong_count(&a) // 3 owners: a, b, c
    );
    {
        let _d = Rc::clone(&a);
        println!(
            "Reference count after creating d: {}",
            Rc::strong_count(&a) // 4 owners: a, b, c, d
        );
    }
    println!(
        "Reference count after d goes out of scope: {}",
        Rc::strong_count(&a) // 3 owners: a, b, c
    );

    // * ----------------------------------------------------------------------------------------------------
    // * Interior mutability (pattern) with `RefCell<T>`
    // Interior mutability = Mutating the value inside an immutable value
    // allows you to mutate data even when there are immutable references to that data
    // ⚠️ normally, this action is disallowed by the borrowing rules
    // the patter uses `unsafe` code inside to bend Rust's usual rules that govern mutation and borrowing
    // Unsafe code tells the compiler "trust me, I know what I'm doing"
    // and the compiler will not check the code for memory safety.
    // let x = 5;
    // let y = &mut x; // ! ❌ can't borrow immutable local variable `x` as mutable
    // (see lib.rs) for practical example

    // * ----------------------------------------------------------------------------------------------------
    // * Allowing Multiple Owners of Mutable Data
    // * Combination of Rc<T> and RefCell<T>
    // ⚠️ For single-thread only
    let a = Rc::new(RefCell::new(String::from("value")));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    b.borrow_mut().push_str(" 🦀");
    c.borrow_mut().push_str(" 🦀");

    println!(
        "The value of 'a' is now: '{}' after b and c borrows and mutate it",
        a.borrow().as_str()
    );

    println!(
        "Reference count after creating a, b, c: {}",
        Rc::strong_count(&a) // 3 owners: a, b, c
    );

    // * ----------------------------------------------------------------------------------------------------
    examples::a_reference_cycle::run();
    examples::prevent_reference_cycle::run();
}
