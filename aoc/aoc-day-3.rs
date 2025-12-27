use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-day-3-input.txt")
        .expect("failed to read input file");

    let mut total_sum: u64 = 0;

    for line in input.lines() {
        let mut result: u64 = 0;
        let mut start_index = 0;
        
        for i in (0..12).rev() {
            let mut current_max_value: u32 = 0;
            let mut bytes_index = start_index;
            for (index, digit) in line.chars().enumerate().skip(start_index) {
                if let Some(digit_val) = digit.to_digit(10) {
                    if index >= line.len() - i {
                        break;
                    }
                    if digit_val > current_max_value {
                        current_max_value = digit_val;
                        bytes_index = index;
                    }
                }
            }
            result = result * 10 + current_max_value as u64;
            start_index = bytes_index + 1;
        }
        total_sum += result;
    }
    println!("{}", total_sum); 
      
}