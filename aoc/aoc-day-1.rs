use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-day-1-input.txt")
        .expect("failed to read input file");
    let mut pwd = 0;
    let mut dial: i32 = 50;

    for rotation in input.lines() {
        let direction = &rotation[0..=0];
        let value: i32 = rotation[1..].trim().parse().expect("failed to parse");
        if direction == "L" {
            pwd += value / 100;
            let remainder = value % 100;
            if dial != 0 && remainder >= dial {
                pwd += 1
            }
            dial = (dial - value).rem_euclid(100);
        } else if direction == "R" {
            pwd += value / 100;
            let remainder = value % 100;
            if dial + remainder >= 100 {
                pwd += 1
            }
            dial = (dial + value) % 100;
        }
    }
    println!("{}", pwd)
}
