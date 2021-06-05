pub fn run() {
    fist_example();
    second_example();
    third_example();
    smart_pointer();
}

fn fist_example() {
    println!("CLOSURE - FIST EXAMPLE");

    let f = |i: i32| i + 1;
    println!("{:?}", f(1));

    let p = || println!("this is a closure");
    p();

    println!("");
}

fn second_example() {
    println!("CLOSURE - SECOND EXAMPLE");

    let mut c = 0;

    let mut inc = || {
        c += 1;
        println!("incremented by 1: {}", c);
    };
    inc();
    inc();
    inc();

    println!("");
}

fn third_example() {
    println!("CLOSURE - THIRD EXAMPLE");
    fn run<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    fn add3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    struct A<F: Fn(i32) -> i32> {
        f: F,
    }

    let p = || println!("hello from run function!");
    run(p);

    let add = |i| i * 10;
    println!("3 * 10 = {}", add3(add));

    let a = A { f: add };
    println!("3 * 10 = {}", add3(a.f));

    println!("");
}

fn smart_pointer() {
    fn create() -> Box<Fn()> {
        Box::new(move || println!("this is a closure in a box!"))
    }

    let x = create();
    x();
}
