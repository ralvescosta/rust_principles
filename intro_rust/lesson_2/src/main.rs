use std::mem;
fn main() {
    tuples();
    array_and_slice();
    slice_of_string();
}

fn tuples() -> () {
    println!("**********Tuples**********");
    let (a, b) = (10, (1, 2, 3));

    println!("{} {:#?}", a, b) // {:?} or {:#?} Debug flag
}

fn array_and_slice() -> () {
    println!("**********Array and Slice**********");
    let array = [4, 5, 6, 7, 8];
    println!("{} {} {}", array[0], array.len(), mem::size_of_val(&array));

    let slice = &array[2..4];
    println!("{:?}", slice);
}

fn slice_of_string() -> () {
    let first = "Fist"; // slice &str
    let second = String::from("Second");

    println!("{} {}", first, second);

    let hello = String::from("Hello, ");
    let world = String::from("World");

    println!("{}", hello + &world);
}
