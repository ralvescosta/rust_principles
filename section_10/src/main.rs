mod traits;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = traits::largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = traits::largest(&char_list);
    println!("The largest char is {}", result);
}