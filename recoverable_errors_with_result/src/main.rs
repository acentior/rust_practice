use std::alloc::System;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let path = "hello.txt";
    // let f = File::open(path).unwrap();
    // let f = File::open(path).expect(&format!("Failed to open {}", path));
    match read_username_from_file() {
        Ok(name) => {
            println!("username is {}", name);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
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
