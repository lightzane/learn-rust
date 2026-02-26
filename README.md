# Packages, Crates and Modules

Reference: <https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html>

## Creating a new Cargo project

### Binary Crate

But if you already have a folder created and want to initialize on current directory:

```sh
cargo new <package_name> # binary crate as default
```

This would generate a **`src/main.rs`**

### Library Crate

```sh
cargo new <package_name> --lib # indicates a library crate
```

This would generate a **`src/lib.rs`** without _`src/main.rs`_

## Lesson Notes

### Packages and Crates

_Crate_ is the smallest amount of code that the Rust compiler considers at a time.

- **Binary** crate - executable programs. It has `src/main.rs` as entry point
- **Library** create - don't have main function. It has `src/lib.rs` as entry point

A _package_ can have mulitple binary crates by placing files in the `src/bin` directory.

If we both have, `src/main.rs` and `src/lib.rs`, we now have 2 crates.

### Modules

By using modules, we can group related definitions together and name why they’re related.

Say you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:

- Inline, within curly brackets that replace the semicolon following `mod garden`
- In the file `src/garden.rs` (paired with `src/garden/` directory)
- In the file `src/garden/mod.rs` (older style, still supported path)
