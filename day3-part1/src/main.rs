use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    println!("===Advent of Code--Day 3, Part 1===");

    let input_file = fs::File::open("input.txt").expect("Could not open `input.txt` for reading");
    match count_duplicate_item_priorities(input_file) {
        Ok(calories) => println!("The sum of duplicate item priorities is {}", calories),
        Err(e) => println!("An error occurred while counting duplicate item priorities. {}", e),
    }
}


fn count_duplicate_item_priorities(input: fs::File) -> Result<u64, String> {
    let reader = BufReader::new(input);
    let mut points = 0u64;

    
'outer: 
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => return Err(format!("Error reading input line {}: {}", index + 1, e.to_string())),
        };

        if line.as_bytes().len() % 2 != 0 {
            return Err(format!("Error on line {}: first and second compartments are not equal length", index + 1))
        }

        let (first, second) = line.as_bytes().split_at(line.len() / 2);

        let mut first_set: u128 = 0; // Bitmap of permissible characters

        for b in first {
            match *b {
                b'a'..=b'z' | b'A'..=b'Z' => first_set |= 1 << *b,
                _ => return Err(format!("Error on line {}: invalid character in compartment", index + 1))
            }
        }

        for b in second {
            match *b {
                b'a'..=b'z' => {
                    if (first_set & (1 << *b)) != 0 {
                        points += (*b - b'a' + 1) as u64;
                        continue 'outer
                    }
                },
                b'A'..=b'Z' => {
                    if (first_set & (1 << *b)) != 0 {
                        points += (*b - b'A' + 27) as u64;
                        continue 'outer
                    }
                },
                _ => return Err(format!("Error on line {}: invalid character in compartment", index + 1))
            }
        }

        // If we've made it here, then none of the itmes in `second` matched those in `first`
        return Err(format!("Error on line {}: no items in first container matched second container", index + 1))
    }

    Ok(points)
}
