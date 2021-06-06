use core::panic;
use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    second_panic(1);
    third_panic(Some(1));
    result_enum_with_pattern_matching();
    result_with_unwrap();
    result_with_expect();
    read_file_with_pattern_matching();
    read_file_with_question_marker();
}

fn first_panic() {
    let v = vec![1, 2];
    println!("{}", v[50]);
}

fn second_panic(x: i32) {
    if x == 0 {
        panic!("we got a 0");
    }
    println!("things are fine!");
    println!();
}

fn third_panic(x: Option<i32>) {
    match x {
        Some(0) => panic!("we got a 0"),
        Some(x) => println!("we gota a {} things are fine!", x),
        None => println!("things are fine!"),
    }
    println!();
}

fn result_enum_with_pattern_matching() {
    let f = File::open("text.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => match File::create("text.txt") {
            Ok(fc) => fc,
            Err(err) => {
                panic!("could not create file: {:?}", err)
            }
        },
        Err(err) => panic!("couldnot open the file {:?}", err),
    };
}

fn result_with_unwrap() {
    let f = File::open("text.txt").unwrap();
}

fn result_with_expect() {
    let f = File::open("text.txt").expect("something went wrong");
}

fn read_file_with_pattern_matching() -> Result<String, io::Error> {
    let f = File::open("text.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

fn read_file_with_question_marker() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("text.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
