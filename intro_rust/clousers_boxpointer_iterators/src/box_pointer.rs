pub fn run() {
    first_example();
    recursive_cons();
}

fn first_example() {
    println!("BOX POINTER - FIRST EXAMPLE");

    let b = Box::new(10);
    println!("b = {:?}", b);

    println!();
}

fn recursive_cons() {
    println!("BOX POINTER - RECURSIVE CONSTRUCTOR");
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        End,
    }

    let l = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::End))))),
    );

    println!("{:?}", l);

    println!("");
}
