use std::{borrow::BorrowMut, collections::HashMap, hash::Hash};

fn main() {
    let mut input = vec![1, 1, 3, 4, 2, 6];
    let median = median(&mut input);
    println!("median is: {:?}", median);
    let mode = mode(&mut input);
    println!("mode is: {:?}", mode);
}

fn median(input: &mut Vec<i32>) -> Option<&i32> {
    let median = 0;
    input.sort();
    let pos = input.len() / 2;
    println!("pos: {}", pos);
    input.get(pos)
}

fn mode(input: &mut Vec<i32>) -> Option<i32> {
    let mut ret: Option<i32> = None;
    if input.len() == 0 {
        ret = None
    } else {
        let mut key = input[0];
        let mut map = HashMap::new();
        for number in input {
            let count = map.entry(number.clone()).or_insert(0);
            *count += 1;
        }

        match map.get(&key) {
            Some(value) => {
                ret = Some(key);
                let mut value = *value;
                for pair in map {
                    if pair.1 > value {
                        ret = Some(pair.0);
                    }
                }
            }
            None => {}
        }
        // assert_eq!(Some(15), Some(14));
        // println!("{}", Some(15).gt(Some(15)));
    }
    ret
    // println!("{:#?}", input);
    // if map.len() == 0 {
    //     ret = None;
    // } else {
    //     let pos = (input[0]).clone();
    //     let key = (map.entry(pos));
    //     let value = 0;
    //     for pair in map {
    //         println!("{:?}", &pair);
    //         // key = pair.0;
    //         // value = pair.1;
    //     }
    // }
    // println!("{:#?}", map);
}
