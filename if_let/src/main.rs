use std::ptr::NonNull;

fn main() {
    // let config_max = Some(3u8);
    let config_max: Option<u8> = None;
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        None => (),
    }
    // match_ex(config_max);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("Nothing");
    }
}

fn match_ex(config_max: Option<u8>) {
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        None => (),
    }
}
