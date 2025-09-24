# Learn Rust

**Officially**, learning [`Rust`](https://www.rust-lang.org/) programming language for the very first time.

References:

- https://doc.rust-lang.org/book/title-page.html
- https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

## Installation

<https://www.rust-lang.org/tools/install>

1. Download `rustup` (recommended)

```sh
# After installation, run in terminal

$ rustup --version
rustup 1.28.2 (e4f3ad6f8 2025-04-28)

$ rustc --version
rustc 1.90.0 (1159e78c4 2025-09-14)

$ cargo --version
cargo 1.90.0 (840b83a10 2025-07-30)
```

| Name     | Description                                           |
| -------- | ----------------------------------------------------- |
| `rustup` | installs and manages `Rust`                           |
| `rustc`  | used to compile `.rs`                                 |
| `cargo`  | **Rust's** package manager just like **Node's** `npm` |

Other terminologies:

- **crates** or cargo libraries (<https://crates.io/>), just like **Node's** packages
- `cargo run` - command similar to `npm start` in Node.js but it will always go for `main.rs` and look for the `main()` function
- `Cargo.toml` - file similar to `package.json`
- `Cargo.lock` - file similar to `package-lock.json` or `pnpm-lock.yaml`

## IDE Integrations

<https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html?highlight=rust-analyzer#ide-integration-using-rust-analyzer>

I'm using **vscode extension**: [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Prerequisites

- https://doc.rust-lang.org/book/ch01-02-hello-world.html
- https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

You should now be able to learn how to print hello world, and by running `rustc main.rs` generates an executable file (`.exe`) for **Windows**, which you can run via command `.\main`

## How this project was created

```sh
cargo new guessing-game
```

This generates a `guessing-game` folder with the following initial files:

- `src/main.rs`
- `.gitignore`
- `Cargo.toml`

Then we add dependency via command (or manually updating `Cargo.toml` and include `rand = "0.9.2"` under `[dependencies]`)

```sh
cargo add rand
```

## Running this project

```sh
# build (debug)
cargo build

# dev (also runs 'cargo build')
cargo run

# build (release)
cargo build --release
```

### Release

In **Windows**, this generates `target\release\guessing-game.exe`

Which you can run by opening the executable directly

Or via command:

```sh
# Windows
.\target\release\guessing-game
```
