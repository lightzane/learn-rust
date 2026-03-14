/*
    Memory leaks is possible to happen in Rust
    by using Rc<T> and RefCell<T>
    where it create references where items refer to each other in a cycle.

    Reference cycles can leak memory.
*/

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Question {
    Sub(&'static str, RefCell<Rc<Question>>),
    Done,
}

use crate::examples::a_reference_cycle::Question::{Done, Sub};

impl Question {
    fn next(&self) -> Option<&RefCell<Rc<Question>>> {
        match self {
            Sub(_, question) => Some(question),
            Done => None,
        }
    }
}

pub fn run() {
    println!("\n\x1b[36;7m Example: Reference Cycles \x1b[0m");

    let a = Rc::new(Sub("Question 1", RefCell::new(Rc::new(Done))));

    println!("\na = {}", question_value(&a));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.next());

    // Rc<T> will "clone" the pointer (not the value it's pointing to)
    let b = Rc::new(Sub("Question 2", RefCell::new(Rc::clone(&a))));

    println!("\nb = {}", question_value(&b));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.next());

    if let Some(sub) = a.next() {
        *sub.borrow_mut() = Rc::clone(&b);
        // ⚠️⚠️⚠️ Not an error but CAUTION is advised
        // b currently holds "a"
        // a is updated to hold "b"
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment either 1 of the next 2 lines to see that we have a cycle;
    // it will overflow the stack.
    // println!("\na = {:?}", a); // if you allow this line, it will loop until STATUS_STACK_OVERFLOW
    // println!("\na next item = {:?}", a.next()); // if you allow this line, it will loop until STATUS_STACK_OVERFLOW
}

fn question_value(question: &Rc<Question>) -> &'static str {
    match question.as_ref() {
        Question::Sub(text, _) => text,
        Question::Done => "",
    }
}
