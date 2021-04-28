fn main() {
    return_ownership_with_reference();
}

fn ownership(){
    let s = String::from("hello");    // s comes into scope

    takes_ownership(s);          // s's value moves into the function...
                                            // ... and so is no longer valid here

    let x = 5;                          // x would move into the function,
                                            // but i32 is Copy, so it’s okay to still
                                            // use x afterward

    makes_copy(x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_ownership(){
    let s = String::from("hello");

    let (s, len) = calculate_length(s);

    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}


fn return_ownership_with_reference(){
    let s = String::from("hello");

    let len = calculate_length_with_ref(&s);

    println!("The length of '{}' is {}.", s, len);
}
fn calculate_length_with_ref(s: &String) -> usize{
    return s.len();
}