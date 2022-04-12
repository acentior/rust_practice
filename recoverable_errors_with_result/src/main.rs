use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "hello.txt";
    // let f = File::open(path).unwrap();
    // let f = File::open(path).expect(&format!("Failed to open {}", path));

    // match read_username_from_file() {
    // match read_username_from_file1() {
    match read_username_from_file1() {
        Ok(name) => {
            println!("username is {}", name);
        }
        Err(err) => {
            println!("Error: {:#?}", err);
        }
    };

    println!(
        "first line last char: {:?}",
        last_char_of_first_line("123123\nwerwer")
    );

    println!("first line last char: {:?}", last_char_of_first_line(""));

    // main return Result<(), E>
    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
