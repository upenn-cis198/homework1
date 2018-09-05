// Part 3.
// Lifetimes and move semantics.

// Problem 1.
// What went wrong? Copy string properly.
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1;
    assert_eq!(str1, str2);
}

// Problem 2.
// How come it works fine here?
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Problem 3.
// These two don't work either. Fix by changing the type of "string" in the function
// copy_me ONLY, and by adjusting the parameter to "copy_me" where it's called.
#[test]
fn eat_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(/* Change in here only*/ str1));
}

#[test]
fn eat_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(str1 /* Change in here only*/);
    assert_eq!(str1, str2);
}

fn copy_me(string : /* Change in here only*/ String) -> String {
    string.clone()
}

// Problem 4.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
fn new_ref_string() -> & String {
    unimplemented!();
}

// Problem 5.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
fn new_ref_str() -> & str {
    unimplemented!();
}

// Problem 6.
// Now we know how to implement this type of function. Implement it and write a test
// pick_longest_tests2() which passes all tests.
fn pick_longest2(s1: & str, s2: & str) -> & str{
    unimplemented!()
}
