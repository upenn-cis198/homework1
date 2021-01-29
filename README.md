# Homework 1: Introduction To Rust.

## Setup

Clone this repository.

You should be be able to run `cargo build` and see no compiler errors or warnings. You should also be able to run `cargo test` and see failing unit tests.

## Assignment

Please modify files `part1.rs`, `part2.rs`, and `part3.rs` as instructed in each problem to have all code compile, and all tests passing. Testing is an important part of the software development process. Hence we expect to write some unit tests when useful and possible.

Use `cargo test` to compile and run the tests. This may also reveal further compiler errors.

## Clippy and Rustfmt

You must ensure that your `clippy` and `rustfmt` are happy with your code! After everything is implemented for a part, also remove the lines
```rust
#![allow(dead_code)]
#![allow(unused_variables)]
```

You are welcome to configure `rustfmt` a bit differently by editing `rustfmt.toml` (e.g. to change max line width). Additionally, if you encounter a case where you think `clippy` or `rustfmt` has it wrong, make a post on Piazza. If I agree, you can disable it for a particular block of code.

## File Structure

The file `main.rs` is what is run when you run `cargo run`, but it also imports a *module* for each part. Every new file creates a new module. Modules are not checked by the compiler unless they're imported. Currently we have in `lib.rs`:

```rust
pub mod part1;
// Uncomment these to have Rust compile the other files as well.
// pub mod part2;
// pub mod part3;
```

Once all tests are working on `part1.rs` uncomment as needed to allow the other module to be checked. This way we avoid having errors from part2.rs or part3.rs stop us from running tests or compiling too much of the working code.

You might notice the `pub` keyword everywhere. In each file, this makes the function public so that `main.rs` has access to it, in case you want to run it there. It also has the benefit that once you remove `#![allow(dead_code)]`, you shouldn't get dead code warnings.

After the first lecture you should be able to complete `part1.rs`.

## Deadline

You have two weeks to do this assignment; it is due Feb 11th at 11:59pm Eastern.
