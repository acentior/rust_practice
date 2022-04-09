use std::io;

fn main() {
    let mut input: String = String::new();
    println!("input n:");
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid farenheit");
    let n: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let nth_fibo = nth_fibonacci(n);
    println!("nth Fibonacci: {}", nth_fibo);
}

fn nth_fibonacci(n: u128) -> u128 {
    let mut ret = 0;
    if n == 0 {
        ret = 0
    } else if n == 1 {
        ret = 1
    } else if n > 1 {
        ret = nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
    }
    ret
}
