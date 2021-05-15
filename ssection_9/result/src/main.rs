use std::{fs, io::Read};
use std::{fs::File};
use std::io::ErrorKind;
use std::io;

fn main() {
    match read_username_and_propagate_error() {
        Err(e) => println!("{:?}", e),
        _ => return,
    };
}

fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn open_file_inner_match() {
    let file_name = "hello.txt";
    let f = File::open(file_name);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name){
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error)
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            },
        },
    };
}

fn open_file_closure() {
    let file_name = "hello.txt";
    let f = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}

fn open_file_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn open_file_expect() {
    let f = File::open("hello.txt").expect("Oi eu sou Goku");
}

fn read_username_and_propagate_error() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_with_question_marker() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_with_question_marker_smaller() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_with_question_marker_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}