# API Documentation

Reference: <https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html>

> [!NOTE]
> Skipping to learn about creating Cargo Accounts and publishing crates

## Lesson Notes

### `cargo build`

```bash
$ cargo build
   Compiling learn-rust v0.1.0 (path\to\learn-rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s

$ cargo build --release
   Compiling learn-rust v0.1.0 (path\to\learn-rust)
    Finished `release` profile [optimized] target(s) in 0.26s
```

The `dev` and `release` are these different profiles used by the compiler.

default values for the `opt-level` setting for the dev and release profiles:

`Cargo.toml`

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

### `cargo doc`

Generate HTML documentation from the documentation comments. (see [src/lib.rs](src/lib.rs))

Documentation comment is indicated by `///` and accepts markdown format.
Also you can run `cargo test` to test the documentation!

```bash
cargo doc
cargo doc --open # opens the browser for you
```

#### Commonly Used Sections

We used the `# Examples` Markdown heading to create a section in the HTML with the title “Examples.” Here are some other sections that crate authors commonly use in their documentation:

- **Panics**: These are the scenarios in which the function being documented could panic. Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.

- **Errors**: If the function returns a `Result`, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so that they can write code to handle the different kinds of errors in different ways.

- **Safety**: If the function is `unsafe` to call, there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

Most documentation comments don’t need all of these sections, but this is a good checklist to remind you of the aspects of your code users will be interested in knowing about.
