# Writing Automated Tests

Reference: <https://doc.rust-lang.org/book/ch11-00-testing.html>

## Lesson Notes

Some command line options go to `cargo test`, and some go to the resultant test binary. To separate these two types of arguments, you list the arguments that go to cargo test followed by the separator `--` and then the ones that go to the test binary. Running `cargo test --help` displays the options you can use with `cargo test`, and `running cargo test -- --help` displays the options you can use after the separator.

### Running tests

#### By default, multiple tests run parallel using threads.

```bash
cargo test
```

Cargo knows how to look for test whether in `#[cfg(test)]` or inside the `tests` folder on the project root directory, next to `src` folder

#### Run tests without parallelism.

```bash
cargo test -- --test-threads=1
```

#### Include println! output on success

By default, when a test passed, the output on `println!` will not be displayed.
This can be altered by running the following command:

```bash
cargo test -- --show-output
```

#### Running a single test

```bash
# cargo test <fn_name>
cargo test it_adds_two
```

#### Running multiple filtered test

```bash
cargo test hold # this will run tests that contains `hold` in the function name
```

#### Running ignored tests only

```bash
cargo test -- --ignored
```

#### Running all tests including ignored

```bash
cargo test -- --include-ignored
```

### Sections in Test

Running `cargo test` may produce different sections. In the example below are (3) sections.

The three sections of output include the unit tests, the integration test, and the doc tests. Note that if any test in a section fails, the following sections will not be run. For example, if a unit test fails, there won’t be any output for integration and doc tests, because those tests will only be run if all unit tests are passing.

```bash
test result: ok. 8 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\integration_test.rs (target\debug\deps\int-cc0c5a046e2afd39.exe)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests learn_rust

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Each integration test file has its own section, so if we add more files in the `tests` directory, there will be more integration test sections.

#### Running integration tests only

```bash
# cargo test --test [filename]
cargo test --test integration_test # will run tests/integration_test.rs
```

#### Run common setup functions or utils for Tests

See [mod.rs](./tests/common/mod.rs)

`tests/**/mod.rs` will not be included in the sections output of `cargo test`
