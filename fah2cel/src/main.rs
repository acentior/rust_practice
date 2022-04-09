use std::io;

fn main() {
    let mut input: String = String::new();
    println!("farenheit");
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid farenheit");
    let mut farenheit: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let celsius = fahrenheit_celsius(farenheit);
    println!("farenheit to celsius: {}", celsius);
    farenheit = celsius_fahrenheit(celsius);
    println!("celsius to farenheit: {}", farenheit);
}

fn fahrenheit_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_fahrenheit(celsius: f64) -> f64 {
    // (fahrenheit - 32.0) * 5.0 / 9.0
    celsius * 9.0 / 5.0 + 32.0
}
