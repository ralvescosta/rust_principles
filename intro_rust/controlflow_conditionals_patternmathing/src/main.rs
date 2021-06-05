fn main() {
    matching_1();
    matching_2();
    matching_3();
    matching_4();
}

fn matching_1() {
    let n = 15;
    match n {
        1 => println!("One"),
        2 | 3 | 4 | 7 | 11 => println!("This a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}

fn matching_2() {
    let pair = (0, -2);

    match pair {
        (0, y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _ => println!("No Match"),
    }
}

fn matching_3() {
    let pair = (5, -5);

    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal zero"),
        (x, _) if x % 2 == 0 => println!("X is even"),
        _ => println!("No Match"),
    }
}

fn matching_4() {
    let p = 5;

    match p {
        n @ 1..=12 => println!("n: {}", n), //bound
        n @ 13..=19 => println!("n: {}", n),
        _ => println!("no match"),
    }
}
