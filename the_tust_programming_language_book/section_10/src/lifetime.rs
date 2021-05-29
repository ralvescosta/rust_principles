use core::fmt::Display;

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    return x;
  }

  return  y;
}

pub fn call_first_ex() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}

pub struct ImportantExcerpt<'a> {
  pub part: &'a str,
}

pub fn call_second_ex() {
  let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

impl<'a> ImportantExcerpt<'a> {
  pub fn level(&self) -> i32 {
      3
  }

  pub fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display,
{ 
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
      x
  } else {
      y
  }
}