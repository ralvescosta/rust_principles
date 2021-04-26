fn main() {
    shadowing();
}

fn first_example(){
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}

fn shadowing(){
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
}