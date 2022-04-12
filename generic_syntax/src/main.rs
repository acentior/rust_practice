// // In Function Definitions
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// In Struct Definitions
struct Point<T> {
    x: T,
    y: T,
}

struct Point1<T, U> {
    x: T,
    y: U,
}

// In Method Definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // // In Function Definitions
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // assert_eq!(result, 100);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
    // assert_eq!(result, 'y');

    // In Struct Definitions
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let both_integer = Point1 { x: 5, y: 10 };
    let both_float = Point1 { x: 1.0, y: 4.0 };
    let integer_and_float = Point1 { x: 5, y: 4.0 };

    // In Method Definitions
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let p = Point { x: 5.0, y: 10.0 };
    println!("p.distance_from_origin = {}", p.distance_from_origin());
}

// In Enum Definitions
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
