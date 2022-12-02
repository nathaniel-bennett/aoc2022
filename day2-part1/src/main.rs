use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    println!("===Advent of Code--Day 2, Part 1===");

    let input_file = fs::File::open("input.txt").expect("Could not open `input.txt` for reading");
    match count_rps_wins(input_file) {
        Ok(calories) => println!("The sum of the rock-paper-scissor points (assuming all wins) is {}", calories),
        Err(e) => println!("An error occurred while counting rock-paper-scissor scores. {}", e),
    }
}


fn count_rps_wins(input: fs::File) -> Result<u64, String> {
    let reader = BufReader::new(input);
    let mut points = 0u64;
    
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => return Err(format!("Error reading input line {}: {}", index + 1, e.to_string())),
        };

        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        if split.len() > 2 {
            return Err(format!("Error reading input: more than 2 values on line {}", index + 1))
        }

        match (split.get(0), split.get(1)) {
            (Some(&"A"), Some(&"X")) => points += 3 + 1,
            (Some(&"A"), Some(&"Y")) => points += 6 + 2,
            (Some(&"A"), Some(&"Z")) => points += 0 + 3,
            (Some(&"B"), Some(&"X")) => points += 0 + 1,
            (Some(&"B"), Some(&"Y")) => points += 3 + 2,
            (Some(&"B"), Some(&"Z")) => points += 6 + 3,
            (Some(&"C"), Some(&"X")) => points += 6 + 1,
            (Some(&"C"), Some(&"Y")) => points += 0 + 2,
            (Some(&"C"), Some(&"Z")) => points += 3 + 3,
            _ => return Err(format!("Error reading input: line {} could not be interpreted as rock paper scissors instructions", index + 1))
        }
    }

    Ok(points)
}
