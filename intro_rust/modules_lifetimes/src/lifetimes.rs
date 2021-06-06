pub fn run() {
    first_example();
    second_example();
    static_lifetime();
}

fn first_example() {
    println!("LIFE TIME - FIRST EXAMPLE");

    fn pr<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() == y.len() {
            return x;
        }

        return y;
    }
    println!("{}", pr("first", "example"));

    print!("");
}

fn second_example() {
    println!("LIFE TIME - SECOND EXAMPLE");

    #[derive(Debug)]
    struct Batata<'a> {
        x: &'a str,
    }
    impl<'a> Batata<'a> {
        fn slf(&self) -> &str {
            self.x
        }
    }
    let k = Batata { x: "Azeda" };
    println!("{:?}, {}", k, k.slf());

    println!("");
}

fn static_lifetime() {
    println!("LIFE TIME - STATIC LIFETIME");

    let s: &'static str = "The Log Living String";
    println!("{}", s);

    println!("");
}
