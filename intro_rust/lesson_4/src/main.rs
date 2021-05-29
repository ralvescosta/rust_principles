use std::fmt::{self, Display};

struct Object {
    width: u32,
    height: u32,
}

impl Object {
    // when we receive in the first argument the &self this func be came a method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

impl Object {
    /*
     when we dont receive &self in the first argument this func be came a static method
     this called Related Functions
    */
    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }
}

// Trais from Struct -> this sintas we can do polimorfismo
impl Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({} , {}) and Area: {}",
            self.width,
            self.height,
            self.area()
        )
    }
}

fn main() {
    let o = Object {
        height: 10,
        width: 10,
    };

    let obj = Object::new(25, 24);

    o.show();
    obj.show();
    println!("{}", o);
    println!("{}", obj);
}
