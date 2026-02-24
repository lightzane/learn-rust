# References and Borrowing

Reference: <https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html>

## How this project was created

(Similar to `cargo new <package_name>`)

But if you already have a folder created and want to initialize on current directory:

```sh
# cargo init [options] [path]
cargo init . # initialize in root/current directory
cargo init --name package_name . # defaults current directory name
```

## Lesson Notes

### The Rules of References

Let’s recap what we’ve discussed about references:

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
