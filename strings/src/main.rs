fn main() {
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating a String
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    // println!("s1 is {}", s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // let s1 = String::from("hello");
    // let h = &s1[0];

    let h = "Здравствуйте";
    let s = &h[0..6];
    println!("h is {}", h);
    println!("s is {}", s);

    let hindi = "नमस्ते";
    println!("{}", hindi);
    for c in hindi.chars() {
        println!("{}", c);
    }
}
