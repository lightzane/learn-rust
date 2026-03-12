#[derive(Debug, PartialEq)] // `PartialEq` is needed for `assert_eq!` for comparison
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    // Remember when returning a value in fn, we need to return the ownership of the value,
    // Since our args doesn't borrow and we need to return a new collection
    // so we can't return an iterator that borrows from the original collection.
    // Hence, we use `into_iter()` instead of `iter()`

    // ? .iter() - borrows immutably (&T)
    // - Iterates over references to elements
    // - the original iterator is not consumed
    // - Use when reading

    // ? .into_iter() - takes ownership (T)
    // - Iterates over elements by value and takes ownership of the original collection
    // - the original iterator is CONSUMED and cannot be used after this point
    // - Use when consuming/transforming

    // ? .iter_mut() - borrows mutably (&mut T)
    // - Iterates over mutable references to elements
    // - the original iterator is not consumed, but we can modify the items it yields
    // - Use when modifying in place
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 12,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        )
    }
}
