use std::fs;
use std::iter::zip;

fn main() {
    let input = fs::read_to_string("input.prod").expect("Failed to read input");
    let lines = input.split('\n');
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let split: Vec<&str> = line.split_whitespace().collect();
        left_list.push(split[0].parse::<i32>().unwrap());
        right_list.push(split[1].parse::<i32>().unwrap());
    }
    left_list.sort();
    right_list.sort();
    let zipped = zip(left_list, right_list);
    let mut distance = 0;
    for (left, right) in zipped {
        distance += (left - right).abs();
    }
    println!("distance {}", distance);
}
