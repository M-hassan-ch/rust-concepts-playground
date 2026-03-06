#![allow(unused)]

fn ret(arr: &[i32]) -> &[i32] {
    &arr[2..]
}
fn main() {
    // Rule 1: Each value has an owner
    // Rule 2: There can be only one owner at a time
    // Rule 3: When the owner goes out of scope, the value will be dropped
    // --------------------------------------------------------------------

    let s = String::from("s");
    let s1 = s;
    // let s2 = s; // Throw error, ownership was already transferred to `s1`

    // Immutable borrow
    let s = String::from("s");
    let s1 = &s;
    let s2 = &s;

    // Mutable reference
    let mut s = String::from("rus");
    let s1 = &mut s;
    s1.push_str("t");
    s.push_str("!");
    println!("{s}");

    // One Mutable reference at a time
    let mut s = String::from("rus");
    let s1 = &mut s;
    // let s2 = &mut s;  // Throw error, one mutable reference at a time
    s1.push_str("t");
    println!("{s}");

    // Cannot take mutable and immutable references simultaneously
    let mut s = String::from("rus");
    let s1 = &mut s;
    // let s2 = &s;  // Throw error
    s1.push_str("t");
    println!("{s}");
    // println!("{s2}");

    // Reference must not outlive its value
    let mut s = String::from("rus");
    let s1 = &s;
    {
        s;
    }
    // println!("{s}"); // throws error
}
