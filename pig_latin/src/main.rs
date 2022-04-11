use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("can not get message");
    let mut inword: String = String::new();
    match input.split_whitespace().next() {
        None => {
            inword = String::from("");
        }
        Some(str) => {
            inword = String::from(str);
        }
    }
    let outword = pig_latin(&inword);
    println!("pit latin is: {}", outword);
    println!("pit latin is: {}", rm_first_char(&inword));
}

fn pig_latin(input: &String) -> String {
    let mut output = String::from(input);
    if input.len() > 0 {
        match input.chars().peekable().next() {
            None => {}
            Some(c) => {
                println!("first is {}", c);
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                        output += "-hay";
                    }
                    'a'..='z' | 'A'..='Z' => {
                        output = format!("{}-{}ay", rm_first_char(&output), c);
                    }
                    _ => {}
                }
            }
        };
    }
    output
}

fn rm_first_char(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
