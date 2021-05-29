fn main() {
    let result = value_in_cents(Coins::Quarter(UsState::Alabama));
    println!("{}", result);

    let result = plus_one(Option::Some(3));
    match result {
        Some(i) =>println!("{}", i),
        _ => (),
    }
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}