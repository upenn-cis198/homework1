# Homework 1 Introduction To Rust.

# Setup
Clone this repository.

You should be be able to run `cargo build` and see several compiler warnings (but no errors).

# Assignment

Please modify files `lib.rs`, `lib2.rs`, and `lib3.rs` as instructed in each problem to have all code compile, and all tests passing. Testing is an important part of the software development process. Hence we expect to write some unit tests when useful and possible.

Use `cargo test` to compile and run the tests, this may reveal further compiler errors.

Every new file creates a new module. Modules are not checked by the compiler unless they're imported. Currently we have:

```rust
// Uncomment these to have Rust compile the other files as well.
// mod lib2;
// mod lib3;
```

Once all tests are working on `lib.rs` uncomment as needed to allow the other module to be checked. This way we avoid having errors from lib2.rs or lib3.rs stop us from running tests or compiling too much of the working code.


After the first lecture you should be able to complete `lib.rs` and `lib2.rs`.

Ensure that your `clippy` and `rustfmt` are happy with your code!

# Submissions
You have two weeks to do this assignment, it is due the September 25th at midnight!
