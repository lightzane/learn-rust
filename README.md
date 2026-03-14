# Smart Pointers

Reference: <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>

> Review: You can _either_ have one **mutable reference** or any number of **immutable references** (but not both).

## Lesson Notes

While **references** only borrow the data, in many cases
**smart pointers** _own_ the data they point to.

Smart pointers are usually implemented using structs.

- **`Box<T>`**, for allocating values on the heap
- **`Rc<T>`**, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through **`RefCell<T>`**, a type that
  enforces the borrowing rules at runtime instead of compile time

`Deref` trait allows you to customize the behavior of the _dereference operator_ `*` (not to be confused with the multiplication or glob operator)
