use core::panic;
use std::{collections::HashMap, fs::File};

fn main() {
    vectors_primitive();
    vector_enum();
    hash_map_primitive();
    if_let();
    while_let();
    casting();
    result_enum();
}

fn vectors_primitive() {
    println!("VECTORS PRIMITIVE");
    let x = vec![1, 2, 3, 4]; // vector is a resizable array
    println!("{:?}", x);

    let mut v = Vec::<u32>::new(); // vector struct has the data, the length and the capacity

    v.push(5);
    v.push(1);
    v.push(10);
    v.push(87);

    for i in &v {
        println!("{}", i);
    }

    println!("{:?} {} {}", &v, v.len(), v.capacity());
    println!("");
}

fn vector_enum() {
    println!("VECTOR ENUM");
    #[derive(Debug)]
    enum Example {
        Float(f64),
        Int(i32),
        Text(String),
    }

    let vec = vec![
        Example::Int(12),
        Example::Float(10.1),
        Example::Text(String::from("String")),
    ];

    println!("{:?}", vec);
    println!("");
}

fn hash_map_primitive() {
    println!("HASH MAP PRIMITIVE");
    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 22);

    for (k, v) in &hm {
        println!("{} : {}", k, v);
    }

    match hm.get("random") {
        Some(&n) => println!("Match: {}", n),
        _ => println!("no match"),
    }

    // hm.remove("strings");

    println!("");
}

fn if_let() {
    println!("IF LET");
    let s = Some('c');

    if let Some(i) = s {
        println!("{}", i);
    }

    println!("");
}

fn while_let() {
    println!("WHILE LET");

    let mut s = Some(0);

    while let Some(i) = s {
        if i > 19 {
            println!("Quit");
            s = None;
        } else {
            println!("{}", i);
            s = Some(i + 2);
        }
    }

    println!("");
}

fn casting() {
    println!("CASTING");

    let f = 24.1414_f32; // cast to float32
    let i = f as u8;
    let c = i as char;

    println!("{}", c);

    println!("");
}

fn result_enum() {
    println!("RESULT ENUM");

    let f = File::open("blablabla.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => panic!("There was a problem opening the file: {:?}", err),
    };

    println!("");
}
