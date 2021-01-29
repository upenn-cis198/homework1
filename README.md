# Homework 1: Introduction To Rust.

## Setup

Clone this repository.

You should be be able to run `cargo build` and see no compiler errors or warnings. You should also be able to run `cargo test` and see failing unit tests.

## Assignment

Please modify files `part1.rs`, `part2.rs`, and `part3.rs` as instructed in each problem to have all code compile, and all tests passing. Testing is an important part of the software development process. Hence we expect to write some unit tests when useful and possible.

Use `cargo test` to compile and run the tests, this may reveal further compiler errors.

## Clippy and Rustfmt

You must ensure that your `clippy` and `rustfmt` are happy with your code! While this is required, you are welcome to configure `rustfmt` a bit differently by editing `rustfmt.toml` (e.g. to change max line width). Additionally, if you encounter a case where you think `clippy` or `rustfmt` has it wrong, make a post on Piazza. If I agree, you can disable it for a particular block of code.

## File Structure

This repository is a *library* rather than a *binary,* which means it has a `lib.rs` instead of a `main.rs`. Practically speaking, that mainly means that you can't run it with `cargo run`, and should instead just write unit tests and run them with `cargo test`. You are welcome to add a `main.rs` file though if you prefer.

Every new file creates a new module. Modules are not checked by the compiler unless they're imported. Currently we have in `lib.rs`:

```rust
mod part1;
// Uncomment these to have Rust compile the other files as well.
// mod part2;
// mod part3;
```

Once all tests are working on `part1.rs` uncomment as needed to allow the other module to be checked. This way we avoid having errors from part2.rs or part3.rs stop us from running tests or compiling too much of the working code.

After the first lecture you should be able to complete `part1.rs`.

## Deadline

You have two weeks to do this assignment; it is due Feb 11th at 11:59pm Eastern.
