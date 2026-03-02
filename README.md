# Error Handling

Reference: <https://doc.rust-lang.org/book/ch09-00-error-handling.html>

Learning about the `panic!` macro and the `Result<T, E>` enum:

```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Lesson Notes

_Rust_ group errors into two major categories:

- **recoverable** with `Result<T, E>` _(e.g file not found)_
- **unrecoverable** with `panic!` _(e.g symptoms of bugs)_

### Unwinding the Stack or Aborting in Response to a Panic

By default, when a panic occurs, the program starts _unwinding_, which means Rust walks back up the stack and cleans up the data from each function it encounters. However, walking back and cleaning up is a lot of work. Rust therefore allows you to choose the alternative of immediately _aborting_, which ends the program without cleaning up.

Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resultant binary as small as possible, you can switch from unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropriate [profile] sections in your _Cargo.toml_ file. For example, if you want to abort on panic in release mode, add this:

```toml
[profile.release]
panic = 'abort'
```

Source: https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic

### Backtrace errrors in Rust

```bash
$ RUST_BACKTRACE=1 cargo run
```

Debug symbols are enabled by default when using `cargo build` or `cargo run` without the `--release` flag,
