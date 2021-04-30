
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30
    };

    let result = area_with_struct(&rect1);

    println!("The area of the rectangle {:?} is {} square pixels.", rect1, result);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    return rectangle.height * rectangle.width;
}