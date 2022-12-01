use std::{collections, fs};
use std::io::{BufRead, BufReader};
use std::cmp::Reverse;

const TOP_ELF_COUNT: usize = 3;

fn main() {
    println!("===Advent of Code--Day 1, Part 2===");

    let input_file = fs::File::open("input.txt").expect("Could not open `input.txt` for reading");
    match count_elf_calories::<TOP_ELF_COUNT>(input_file) {
        Ok(calories) => println!("The sum of calories being carried by the top {} elves is: {}", TOP_ELF_COUNT, calories),
        Err(e) => println!("An error occurred while counting elf calories. {}", e),
    }
}


fn count_elf_calories<const T: usize>(input: fs::File) -> Result<u64, String> {
    let reader = BufReader::new(input);
    let mut max_calorie_counts = collections::BinaryHeap::from([Reverse(0u64); T]);
    let mut curr_calorie_count = 0u64;
    
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => return Err(format!("Error reading input line {}: {}", index + 1, e.to_string())),
        };
        
        if line.is_empty() {
            max_calorie_counts.push(Reverse(curr_calorie_count)); // Reverse() reverses the comparison, making this a min heap
            max_calorie_counts.pop();
            curr_calorie_count = 0;

        } else {
            let calorie_count: u64 = match line.parse() {
                Ok(cnt) => cnt,
                Err(e) => return Err(format!("Couldn't parse calorie value at line {}: {}", index + 1, e.to_string())),
            };
            
            curr_calorie_count += calorie_count as u64;
        }
    }

    Ok(max_calorie_counts.into_iter().map(|r| r.0).sum())
}
