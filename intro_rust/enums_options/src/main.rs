#![allow(dead_code)] // annotation

fn main() {
    basic_enums();
    enum_impl();
}

fn basic_enums() {
    enum Direction {
        Up(Point),
        Down(Point),
        Left(Point),
        Right(Point),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let u = Direction::Up(Point { x: 0, y: 0 });
}

fn enum_impl() {
    #[derive(Debug)] // debug flag
    enum Direction {
        Up(Point),
        Down(Point),
        Left(Point),
        Right(Point),
    }

    #[derive(Debug)] // debug flag
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)] // debug flag
    enum Keys {
        UpKey(String),
        DownKey(String),
        LeftKey(String),
        RightKey(String),
    }

    impl Direction {
        fn match_direction(&self) -> Keys {
            match *self {
                Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
                Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
                Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
                Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
            }
        }
    }

    impl Keys {
        fn destruct(&self) -> &str {
            match *self {
                Keys::UpKey(ref s) => s,
                Keys::DownKey(ref s) => s,
                Keys::LeftKey(ref s) => s,
                Keys::RightKey(ref s) => s,
            }
        }
    }

    let u = Direction::Up(Point { x: 0, y: 1 });
    let k = u.match_direction();
    let x = k.destruct();

    println!("{:?}, {}", k, x);
}
