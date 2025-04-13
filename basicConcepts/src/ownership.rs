//Ownership
//Every value has a single owner.[Every variable has one value, and it is its sole owner.]
//Ownership rules:
//1. Each value in Rust has an owner.
//2. There can only be one owner at a time.
//3. When the owner goes out of scope, the value will be dropped.

pub fn ownership() {
    first_rule();
    second_rule();
    third_rule();
}

fn first_rule() {
    let s1 = String::from("RUST"); // s1 is the owner of the string
    let len = calculate_lenght(&s1); // s1 is borrowed by reference
    println!("The length of {} is {}", s1, len); // s1 is still valid
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}

fn second_rule() {
    let s1 = String::from("RUST"); // s1 is the owner of the string
    let s2 = s1.clone();
    println!("s1: {}", s1); // s1 is still valid
    println!("s2: {}", s2); // s2 is a clone of s1
}
// s1 goes out of scope here, so the value is dropped
// fn print_lost(s: &String) {
//     println!("s: {}", s1); //cannot find value `s1` in this scope
// }

fn third_rule() {
    let s1 = String::from("RUST"); // s1 is the owner of the string
    let len = calculate_lenght(&s1); // s1 is borrowed by reference
    println!("The length of {} is {}", s1, len); // s1 is still vali
}
