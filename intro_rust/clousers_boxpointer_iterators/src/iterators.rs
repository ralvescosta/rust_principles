pub fn run() {
    first_example();
    second_example_imperative();
    second_example_functional();
}

fn first_example() {
    println!("ITERATORS - FIRST EXAMPLE");

    let v = vec![3, 4, 10];
    println!("v {}", v.iter().any(|&x| x != 2));
    for i in v.iter() {
        println!("{}", i);
    }

    println!("");
}

fn second_example_imperative() {
    println!("ITERATORS - SECOND EXAMPLE IMPERATIVE");

    fn is_even(n: u32) -> bool {
        return n % 2 == 0;
    }

    let top = 10000;
    let mut c = 0;

    for n in 0.. {
        let x = n * n;

        if x >= top {
            break;
        } else if is_even(x) {
            c += x;
        }
    }
    println!("{}", c);

    println!("");
}

fn second_example_functional() {
    println!("ITERATORS - SECOND EXAMPLE FUNCTIONAL");

    fn is_even(n: u32) -> bool {
        return n % 2 == 0;
    }

    let s = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < 10000)
        .filter(|&n| is_even(n))
        .fold(0, |s, i| s + i);
    println!("{}", s);

    print!("");
}
