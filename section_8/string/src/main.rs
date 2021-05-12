use std::ops::Add;
fn main() {
    let s = String::new();
    let s = s.add("rhs");

    println!("{:?}", s);
    println!();


    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents".to_string();
    println!("{}", s);
    println!();


    let hello = String::from("السلام عليكم");
    println!("{}", hello);

    let hello = String::from("Dobrý den");
    println!("{}", hello);

    let hello = String::from("Hello");
    println!("{}", hello);

    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);

    let hello = String::from("नमस्ते");
    println!("{}", hello);

    let hello = String::from("こんにちは");
    println!("{}", hello);

    let hello = String::from("안녕하세요");
    println!("{}", hello);

    let hello = String::from("你好");
    println!("{}", hello);

    let hello = String::from("Olá");
    println!("{}", hello);

    let hello = String::from("Здравствуйте");
    println!("{}", hello);

    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}", s1);
    println!();

    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!();

    let s1 = String::from("tic");
    let s1 = String::from("tac");
    let s1 = String::from("toe");
    let s = s1 + "-" + &s2 + &s3;
    println!();

    let s1 = String::from("tic");
    let s1 = String::from("tac");
    let s1 = String::from("toe");
    let s = format!("{}-{}-{}", s1,s2,s3);
    println!();

    let ba = "mandioca";
    println!("{}", &ba[0..1]);
    println!();

    for c in "mandioca".chars() {
        println!("{}", c);
    }
    println!();

    for b in "mandioca".bytes() {
        println!("{}", b);
    }
}
