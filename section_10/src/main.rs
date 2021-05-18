mod lifetime;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = lifetime::longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}