# Ownership

Reference: <https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html>

Learning _Ownership_ that it's a set of rules that govern how Rust program manages memory.

Bonus advance learning about _Lifetime_. See ([lifetime.rs](src/lifetime.rs))

## How this project was created

(Similar to `cargo new <package_name>`)

But if you already have a folder created and want to initialize on current directory:

```sh
# cargo init [options] [path]
cargo init . # initialize in root/current directory
cargo init --name package_name . # defaults current directory name
```

## Lesson Notes

### 💡 Stack & Heap are parts of memory

| Memory    | 💾 Size      | ⚡ Speed         |
| --------- | ------------ | ---------------- |
| **Stack** | fixed size   | faster to access |
| **Heap**  | dynamic size | slower to access |

#### Stack

- Stores fixed-size data (e.g., integers, floats, booleans)
- Last In, First Out (LIFO) structure - Fast access, but limited in size (usually a few MBs)
- Think of a stack of plates: When you add more plates,
  you put them on top of the pile, and when you need a plate, you take one off the top.

#### Heap

- Stores dynamic-size data (e.g., strings, vectors, structs)
- No specific order, managed by the allocator
- Slower access, but can grow in size (limited by system memory)
- Think of being seated at a restaurant.
  When you enter, you state the number of people in your group,
  and the host finds an empty table that fits everyone and leads you there.
  If someone in your group comes late, they can ask where you’ve been seated to find you.
