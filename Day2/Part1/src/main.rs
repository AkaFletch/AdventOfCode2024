use std::fs;

#[derive(Copy, Clone)]
enum Direction {
    Starting,
    Increasing,
    Decreasing,
}

fn main() {
    println!("Day 2 Part 1");
    let input = fs::read_to_string("input.prod").expect("Failed to read input");
    let mut safe_report = 0;
    'report: for report in input.split('\n') {
        if report.is_empty() {
            continue;
        }
        let list: Vec<i32> = report
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut last = list[0];
        let mut dir = Direction::Starting;
        let mut dif: i32;
        let mut diff_check: bool;
        for (i, level) in list.iter().enumerate() {
            let index_direction = (i, dir);
            dif = (last - *level).abs();
            diff_check = (1..=3).contains(&dif);
            match index_direction {
                (_, Direction::Increasing) => {
                    if last >= *level || diff_check {
                        continue 'report;
                    }
                }
                (_, Direction::Decreasing) => {
                    if *level >= last || diff_check {
                        continue 'report;
                    }
                }
                (1, _) => {
                    if diff_check {
                        continue 'report;
                    }
                    dir = if last < *level {
                        Direction::Increasing
                    } else {
                        Direction::Decreasing
                    };
                }
                (_, Direction::Starting) => {}
            }
            last = *level;
        }
        safe_report += 1;
    }
    println!("Safe reports {}", safe_report);
}
