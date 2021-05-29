fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30
    };
    println!("The area of the rectangle {:?} is {} square pixels.", rect1, rect1.area());

    let rect2 = Rectangle::square(10);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    return rectangle.height * rectangle.width;
}