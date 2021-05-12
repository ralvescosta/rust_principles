mod front_of_house;
mod back_of_house;

use front_of_house::hosting as batata;

pub fn eat_at_restaurant() {
    batata::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// --snip--
// use std::cmp::Ordering;
// use std::io;
// --snip--
// --snip--
use std::{cmp::Ordering, io};
// --snip--
