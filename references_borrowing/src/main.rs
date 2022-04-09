fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    let r1 = &s1; // no problem
    let r2 = &s1; // no problem
    println!("{} and {}", r1, r2);

    let r3 = &mut s1;
    println!("{}", r3);

    // let reference_to_nothing = dangle();
    let reference_something = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
