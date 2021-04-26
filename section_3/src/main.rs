fn main() {
    tuples();
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

fn conversion(){
    let guess = "42".parse::<u32>().expect("Not a number!");
    println!("The guess number is {}", guess);
}

fn integers() {
    let literal = 98_222;
    println!("The literal number is {}", literal);

    let literal = 0xff;
    println!("The literal number is {}", literal);

    let literal = 0o77;
    println!("The literal number is {}", literal);

    let literal = 0b1111_0000;
    println!("The literal number is {}", literal);

    let literal = b'F';
    println!("The literal number is {}", literal);
}

fn floating(){
    let x = 2.0;
    println!("The float number is {:?}", x);

    let y: f32 = 2.0;
    println!("The float number is {:?}", y);
}

fn character(){
    let c = 'x';
    let z = 'z';
    let heart_eyed_cat = '😻'; 
}

fn tuples() -> (i32, f64, char) {
    let tup = (500, 6.4, '1');

    println!("The first is: {:?}", tup.0);
    println!("The second is: {:?}", tup.1);
    println!("The third is: {:?}", tup.2);
    return tup;
}