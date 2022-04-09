use std::io;

fn main() {
    // Scalar Types
    // Integer types
    let x: i64 = 98_222;
    println!("value of x is {}, type is {}", x, type_of(&x));
    let x: u64 = 0xff;
    println!("value of x is {}, type is {}", x, type_of(&x));
    let x: isize = 0o77;
    println!("value of x is {}, type is {}", x, type_of(&x));
    let x: usize = 0b1111_0000;
    println!("value of x is {}, type is {}", x, type_of(&x));
    let x: u8 = b'A';
    println!("value of x is {}, type is {}", x, type_of(&x));

    // Floating-Point Types
    let x = 2.0;
    println!("value of x is {}, type is {}", x, type_of(&x));
    let x: f32 = 3.0;
    println!("value of x is {}, type is {}", x, type_of(&x));

    // Numeric Operations
    // addition
    let sum = 5 + 10;
    println!("sum is {}, type is {}", sum, type_of(&sum));

    // subtraction
    let difference = 95.5 - 4.3;
    println!(
        "difference is {}, type is {}",
        difference,
        type_of(&difference)
    );

    // multiplication
    let product = 4 * 30;
    println!(
        "multiplication is {}, type is {}",
        product,
        type_of(&product)
    );

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient is {}, type is {}", quotient, type_of(&quotient));
    let floored = 2 / 3; // Results in 0
    println!("floored is {}, type is {}", floored, type_of(&floored));

    // remainder
    let remainder = 43 % 5;
    println!(
        "remainder is {}, type is {}",
        remainder,
        type_of(&remainder)
    );

    // Boolean Type
    let t = true;
    println!("t is {}, type is {}", t, type_of(&t));

    let f: bool = false; // with explicit type annotation
    println!("f is {}, type is {}", f, type_of(&f));

    // The Character Type
    let c = 'z';
    println!("c is {}, type is {}", c, type_of(&c));
    let z = 'ℤ';
    println!("z is {}, type is {}", z, type_of(&z));
    let heart_eyed_cat = '😻';
    println!(
        "heart_eyed_cat is {}, type is {}",
        heart_eyed_cat,
        type_of(&heart_eyed_cat)
    );

    // Compound types
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is {:?}, type is {}", tup, type_of(&tup));

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "five is {}, type is {}",
        five_hundred,
        type_of(&five_hundred)
    );
    println!(
        "six_point_four is {}, type is {}",
        six_point_four,
        type_of(&six_point_four)
    );
    println!("one is {}, type is {}", one, type_of(&one));

    // array
    let a = [1, 2, 3, 4, 5];
    println!("a is {:?}, type is {}", a, type_of(&a));
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months is {:?}, type is {}", months, type_of(&months));
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a is {:?}, type is {}", a, type_of(&a));
    let first = a[0];
    let second = a[1];
    println!("first is {:?}, type is {}", first, type_of(&first));
    println!("second is {:?}, type is {}", second, type_of(&second));

    let a = [3; 5];
    println!("a is {:?}, type is {}", a, type_of(&a));

    // Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

// return the type
fn type_of<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}
