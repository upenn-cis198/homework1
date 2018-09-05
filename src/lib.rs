#![allow(dead_code)]

use std::path::Path;
use std::fs::File;
use std::io::Read;

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
fn filter(vs: & Vec<i32>, pred: & Fn(i32) -> bool) -> Vec<i32> {
    unimplemented!()
}

#[test]
fn filter_tests(){
    assert_eq!(filter(& vec![1, 2, 3, 4, 5, 6], & |n| n % 2 == 0),
              vec![2, 4, 6]);
}
