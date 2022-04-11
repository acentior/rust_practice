use std::{fs::File, io::ErrorKind, panic};

fn main() {
    let path = "hello.txt";
    // let f = File::open(path).unwrap();
    let f = File::open(path).expect(&format!("Failed to open {}", path));
}
