#![allow(dead_code)]

// Uncomment these to have Rust compile the other files as well.
// mod lib2;
// mod lib3;

// Part 1. Implementing Functions. Taken from Fall 2016's Rust class.
// Write unit tests for you functions.

// Problem 1
// Implement the sum function on slices. Do not use the predefined sum function.
fn sum(slice: &[i32]) -> i32 {
    unimplemented!()
}

// Problem 2.
// Make unique. Create a new vector which contains each item in the vector
// only once! Much like a set would.
// Please implement this using a for loop.
fn unique(vs: &Vec<i32>) -> Vec<i32> {
    unimplemented!()
}

// Problem 3.
// return a new vector containing only elements that satisfy `pred`.
fn filter(vs: & Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    unimplemented!()
}

#[test]
fn filter_tests(){
    assert_eq!(filter(& vec![1, 2, 3, 4, 5, 6], & |n| n % 2 == 0),
              vec![2, 4, 6]);
}


// Problem 4
// Given starting fibonacci numbers n1 and n2, compute a vector
// where v[i] is the ith fibonacci number.
fn fibonacci(n1: i32, n2: i32, how_many: usize) -> Vec<i32> {
    unimplemented!()
}

// Problem 5
// Create a function which concats 2 strs and returns a String.
// You may use any standard library function you wish.
fn str_concat(s1: &str, s2: &str) -> String {
    unimplemented!()
}

// Problem 6
// Create a function which concats 2 string and returns a String.
// You may use any standard library function you wish.
fn string_concat(s1: &String, s2: &String) -> String {
    unimplemented!()
}

// Problem 7
// Convert a Vec<String> into a Vec<u64>. Assume all strings
// are correct numbers! We will do error handling later. Use
// `.expect("ignoring error")` to ignore Result from parse()
// See https://doc.rust-lang.org/std/primitive.str.html#method.parse
// Use turbo fish! Do not use type inference for parse()
fn concat_all(v: Vec<String>) -> Vec<u64> {
    unimplemented!()
}

// Implement concat_all using map, parse (with turbo fish), and collect()
// Check out how the lecture does something similar:
// https://github.com/upenn-cis198/lecture2/blob/f54753527c1dabbd5e55c2f48a19745768769beb/src/lib.rs#L362
fn concat_all_with_map(v: Vec<String>) -> Vec<u64> {
    unimplemented!()
}
