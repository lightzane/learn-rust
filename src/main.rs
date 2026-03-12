fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    /*
        Iterators are lazy.
        v1_iter doesn't do anything until we call a method that consumes the iterator,
        like `sum` or `collect`.
    */
    for val in v1_iter {
        println!("Got: {}", val);
    }
    // println!("The iterator value is now: {v1_iter:?}"); // ! ❌ v1_iter is moved into the for loop and cannot be used here
    /* The `for` loop will take ownership of v1_iter and iterate over it */

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    println!("v1_iter: {:?}", v1_iter.next());
    println!("v1_iter: {:?}", v1_iter.next());
    println!("v1_iter: {:?}", v1_iter.next());
    println!("The iterator value is now: {v1_iter:?}");
    /* Meanwhile `.next()` does not take ownership of the iterator */
    /* `.next()` consumes the iterator */

    // * Iterator Adapters (doesn't consume the iterator, returns new Iterators)
    let v1 = vec![1, 2, 3];
    v1.iter().map(|x| x + 1); // No error, but doesn't do anything because `map` is lazy and returns a new iterator that we don't use
    // 👆🏻 The above line have warnings that map is unused,
    // to solve this we'll do the following:
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2); // prints: `v2: [2, 3, 4]`
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}
