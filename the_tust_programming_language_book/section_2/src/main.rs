use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        match io::stdin().read_line(&mut guess){
            Ok(_) => {
                println!("You guessed: {}", guess)
            }
            Err(err) => println!("Something wrong in your input: {}", err),
        }

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Great, you win!");
                break;
            },
        }
    }
}
