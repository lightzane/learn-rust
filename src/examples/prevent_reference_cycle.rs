use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[allow(unused)]
#[derive(Debug)]
// * Step 1:
// struct Node {
//     // We want `Node` to own its `children`
//     // And we want to share that ownership with variables
//     // So that we can access each `Node` in the tree directly
//     value: i32,
//     children: RefCell<Vec<Rc<Node>>>,
// }

// * Step 2:
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // + to enable child have a reference to its parent
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn run() {
    println!("\n\x1b[36;7m Example: Preventing Reference Cycles \x1b[0m");

    // let leaf = Rc::new(Node {
    //     value: 3,
    //     children: RefCell::new(vec![]),
    // });

    // let branch = Rc::new(Node {
    //     value: 5,
    //     children: RefCell::new(vec![
    //         // 1st child
    //         Rc::clone(&leaf),
    //     ]),
    // });

    /*
        👆🏻 Since above setup
        The `Node` in `leaf` variable has 2 owners:
            - `leaf`
            - `branch`

        But the `leaf` doesn't have a reference to its parent
    */

    // * -----------------------------------------------------------------
    // * Adding a reference from Child to its Parent
    // * We will update `Node` struct definition to include `parent` field
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![
            // first child
            Rc::clone(&leaf),
        ]),
    });

    // Rc::clone() = ownership + increases `Rc::strong_count`
    // Rc::downgrade() = NO ownership + increase `Rc::weak_count`
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf rc strong count = {}", Rc::strong_count(&leaf)); // 2
    println!("leaf rc weak count = {}", Rc::weak_count(&leaf)); // 0

    println!("branch rc strong count = {}", Rc::strong_count(&branch)); // 1
    println!("branch rc weak count = {}", Rc::weak_count(&branch)); // 1

    // Weak<T> does not keep alive.
    // Value can be dropped if weak references still exist
    // To use a weak reference
    // we have to call .upgrade() which returns Option<Rc<T>> because the value might already be gone
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // * -----------------------------------------------------------------
    // * Visualizing changes to `strong_count` and `weak_count`
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // internal scope (just in case you didn't notice)
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
