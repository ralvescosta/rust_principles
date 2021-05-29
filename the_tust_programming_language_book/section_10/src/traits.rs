use core::fmt::Debug;
use core::fmt::Display;

pub fn invoke() {
  let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }

  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

fn call() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());
}

pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

pub fn notify_2<T: Summary>(item1: T, item2: T) {
  println!("Breaking news 1! {}", item1.summarize());
  println!("Breaking news 2! {}", item2.summarize());

}

pub fn notify_3<T: Summary + Display>(item: &T) {}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
  return 2;
}

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}

pub struct Pair<T> {
  x: T,
  y: T
}

impl<T> Pair<T> {
  pub fn new(x: T, y: T) -> Self {
    Self { x, y}
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  pub fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest number is x = {}", self.x);
    } else {
      println!("The largest number is y = {}", self.y);
    }
  }
}