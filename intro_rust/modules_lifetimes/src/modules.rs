pub fn run() {
    first_example();
    second_example();
}

fn first_example() {
    println!("MODULES - FIRST EXAMPLE");

    mod a {
        pub fn a() {}
        pub fn b() {}
        pub mod b {
            pub fn a() {}
            pub fn b() {}
        }
    }

    a::a();
    a::b();
    a::b::a();
    a::b::b();

    println!();
}

fn second_example() {
    println!("MODULES - SECOND EXAMPLE");

    pub mod a {
        pub mod b {
            pub mod c {
                pub mod d {
                    pub fn e() {}
                }
            }
        }
    }

    use a::b::c::d;
    d::e();

    println!("");
}
