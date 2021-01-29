/*
    CIS198 Homework 1
    Part 2: Strings, files, and mutability

    Make the following failing functions/tests pass.
    Answer the questions as a comment next to the problems.
*/

// Remove these once you are done editing the file!
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;

/*
    Problem 1: Split variants

    Create functions split_ref and split_clone such that
    all the following tests will pass. Feel free to use Rust's split method
    (https://doc.rust-lang.org/std/primitive.slice.html#method.split)
    as needed.
*/

// split_ref must have the return type Vec<&str>
// split_clone must have the return type Vec<String>

// #[test]
// fn test_split_ref(){
//     let string = "Hello World!".to_string();
//     assert_eq!(split_ref(& string), ["Hello", "World!"]);
//     assert_eq!(split_ref("Hello World!"), & ["Hello", "World!"]);
//     assert_eq!(split_ref("Hello World!"), vec!["Hello", "World!"]);
// }

// #[test]
// fn test_split_clone(){
//     let string = "Hello World!".to_string();
//     assert_eq!(split_clone(& string), ["Hello", "World!"]);
//     assert_eq!(split_clone("Hello World!"), & ["Hello", "World!"]);
//     assert_eq!(split_clone("Hello World!"), vec!["Hello", "World!"]);
// }

/*
    Problem 2: Longest string

    Write function pick_longest which picks the longests of two &str arguments.
    Taking &str arguments makes it more general than taking Strings.
    Return a new String (we will see later how to return a &str.)
*/

// #[test]
// fn test_pick_longest {
//     assert_eq!(
//         pick_longest(& "cat".to_string(), & "dog".to_string()),
//         "cat".to_string()
//     );
// }

// Question 1:
// For the curious, attempt to return reference, that is:
//
// fn pick_longest(s1: &str, s2: &str) -> &str
//
// What goes wrong when you try to implement this function? Why is this
// the case?

/*
    Problem 3: File to string

    Write a function that returns all the contents of a file as a single String.

    DO NOT USE the assocated function std::fs::read_to_string

    Instead use File::open, and the method read_to_string
    (https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string)

    You can use .expect("ignoring error: ") to ignore the Result<...> type in open()
    and read_to_string. We will discuss error handling later.
*/

pub fn file_to_string(path: &str) -> String {
    unimplemented!()
}

/*
    Problem 4: Mutability

    Why does the following implementation not work as expected?
    Fix by changing the type signature of add1 and the way it's called on add1_test().
    do NOT change the return type.
*/

#[test]
fn test_add1() {
    let mut x = 1;
    add1(x);
    assert_eq!(x, 2);
}

pub fn add1(mut x : i32) -> () {
    x += 1;
}

/*
    Problem 5: Mutability continued

    The error says: cannot assign to immutable borrowed content `*str1`
    But we declared it mutable? Fix by changing only the line below.
*/
// pub fn mut2() {
//     let hello = String::from("hello");
//
//     // CHANGE ONLY THIS LINE:
//     let mut str1: &String = &String::from("str1");
//
//     *str1 = hello;
// }
