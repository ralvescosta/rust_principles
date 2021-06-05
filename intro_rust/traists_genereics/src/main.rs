fn main() {
    trait_first();
    trait_second();
}

fn trait_first() {
    println!("FIRST");
    trait Shape {
        fn area(&self) -> u32;
    }

    struct Rectangle {
        x: u32,
        y: u32,
    }

    struct Circle {
        radius: f64,
    }

    impl Shape for Rectangle {
        fn area(&self) -> u32 {
            self.x * self.y
        }
    }

    impl Shape for Circle {
        fn area(&self) -> u32 {
            (3.14 * self.radius * self.radius) as u32
        }
    }

    let re = Rectangle { x: 10, y: 10 };
    let circ = Circle { radius: 12.1 };

    println!("{}, {}", re.area(), circ.area());

    println!("");
}

fn trait_second() {
    println!("SECOND");

    #[derive(Debug, Clone, Copy)]
    struct A(i32);

    // #[derive(Eq, PartialEq, PartialOrd, Ord)]
    struct B(f32);

    let a = A(32);
    let c = a;

    println!("{:?}, {:?}", a, c);
}
