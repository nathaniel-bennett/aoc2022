use std::{cmp, fs};
use std::io::{BufRead, BufReader};

fn main() {
    println!("===Advent of Code--Day 1, Part 1===");

    println!("Reading input...");
    let input_file = fs::File::open("input.txt").expect("Could not open `input.txt` for reading");
    let reader = BufReader::new(input_file);

    let mut max_calorie_count = 0u128;
    let mut curr_calorie_count = 0u128;
    
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect(format!("Error reading input at line {}", index + 1).as_str());

        if line.is_empty() {
            max_calorie_count = cmp::max(max_calorie_count, curr_calorie_count);
            curr_calorie_count = 0;

        } else {
            let calorie_count: u32 = line.parse().expect("Non-integer number (or out of bounts number) encountered while parsing lines");
            curr_calorie_count += calorie_count as u128;
        }
    }

    println!("The highest number of calories being carried by an elf is {}", max_calorie_count);
}
