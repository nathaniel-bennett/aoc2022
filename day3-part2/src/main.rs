use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    println!("===Advent of Code--Day 3, Part 2===");

    let input_file = fs::File::open("input.txt").expect("Could not open `input.txt` for reading");
    // 3 elves per group
    match count_common_item_priorities(input_file, 3) {
        Ok(calories) => println!("The sum of duplicate item priorities is {}", calories),
        Err(e) => println!("An error occurred while counting duplicate item priorities. {}", e),
    }
}


fn count_common_item_priorities(input: fs::File, num_elves: usize) -> Result<u64, String> {
    let reader = BufReader::new(input);
    let mut lines = reader.lines().peekable();
    let mut points = 0u64;
    
    if num_elves < 2 {
        return Err("Not enough elves to find common items".to_string())
    }

    while lines.peek().is_some() {
        let mut combined_set: u128 = u128::MAX; // All 1 bits

        for _ in 0..num_elves {
            let mut curr_set: u128 = 0;
            let line = match lines.next() {
                Some(Ok(l)) => l,
                Some(Err(e)) => return Err(format!("Error retrieving line: {}", e.to_string())),
                None => return Err("Insufficient number of elves in the last group".to_string()),
            };

            for b in line.as_bytes() {
                match *b {
                    b'a'..=b'z' | b'A'..=b'Z' => curr_set |= 1 << *b,
                    _ => return Err("Invalid character detected".to_string()),
                }
            }

            combined_set &= curr_set;
        }

        if combined_set == 0 {
            return Err("No common item among elves in the group".to_string())
        }

        let common_value = combined_set.trailing_zeros() as u8;
        points += match common_value {
            b'a'..=b'z' => (common_value - b'a' + 1) as u64,
            b'A'..=b'Z' => (common_value - b'A' + 27) as u64,
            _ => return Err(format!("Invalid character {}", common_value)),
        }
    }

    Ok(points)
}

