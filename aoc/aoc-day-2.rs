use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-day-2-input.txt")
        .expect("failed to read input file");
    let mut sum = 0;
    let ranges: Vec<&str> = input.split(',').collect();
    let mut total_ranges: Vec<i64> = Vec::new();
    for range in &ranges {
        let part: Vec<&str> = range.split('-').collect();
        let start: i64 = part[0].parse().expect("failed to parse start");
        let end: i64 = part[1].parse().expect("failed to parse end");
        let temp_range: Vec<i64> = (start..=end).collect();
        total_ranges.extend(temp_range);
    }

    for value in &total_ranges {
        let s = value.to_string();
        let n = s.len();
        for i in 2..n+1 {
            if n % i == 0 && s[..n/i].repeat(i) == s {
                sum += value;
                break
            }
        }
    }

    println!("{}", sum)
}