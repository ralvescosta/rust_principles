pub fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item
    }
  }
  
  largest
}

struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point<f32, f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
      Point {
          x: self.x,
          y: other.y,
      }
  }
}

fn instance_of_point() {
  let both_integer = Point { x: 5, y: 10.1 };
}