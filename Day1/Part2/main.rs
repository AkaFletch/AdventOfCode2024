use std::fs;

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
    let mut score = 0;
    for left in left_list {
        score += left * right_list.iter().filter(|x| **x == left).count() as i32;
    }
    println!("score {}", score);
}
