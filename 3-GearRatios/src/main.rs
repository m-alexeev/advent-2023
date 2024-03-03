use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SYMBOLS: &str = "*#-+@%&=$/";

fn _get_symbols(buffer: &Vec<String>) -> String {
    let mut symbols = String::from("");
    for line in buffer {
        for char in line.chars() {
            if !symbols.contains(char) {
                symbols.push(char);
            }
        }
    }
    symbols
}

fn part1(buffer: &Vec<String>) -> u32 {
    let mut part_sum = 0;
    for (i, line) in buffer.iter().enumerate() {
        let mut current_num: String = String::from("");
        let mut can_add = false;
        for (j, char) in line.chars().enumerate() {
            // println!("Char: {}", char);
            match char.to_string().parse::<u32>() {
                Ok(_) => {
                    for x in -1..2 {
                        for y in -1..2 {
                            let row_index = i as i32 + x;
                            let col_index = j as i32 + y;
                            if x == 0 && y == 0 {
                                continue;
                            }
                            // Checks surrounding cells
                            if row_index >= 0
                                && row_index < buffer.len() as i32
                                && col_index >= 0
                                && col_index < buffer.len() as i32
                            {
                                let symbol = buffer[row_index as usize]
                                    .chars()
                                    .nth(col_index as usize)
                                    .unwrap();
                                if SYMBOLS.contains(symbol) {
                                    can_add = true;
                                }
                            }
                        }
                    }
                    current_num.push(char)
                }
                Err(_) => {
                    if !current_num.is_empty() {
                        if can_add {
                            // println!("{:?}", current_num.parse::<u32>().unwrap());
                            part_sum = part_sum + current_num.parse::<u32>().unwrap();
                        }
                        current_num = String::from("");
                        can_add = false;
                    }
                }
            }
        }
        if !current_num.is_empty() {
            if can_add {
                part_sum = part_sum + current_num.parse::<u32>().unwrap();
            }
        }
    }
    part_sum
}

fn part2(buffer: &Vec<String>) -> u32 {
    let mut gear_dict: HashMap<(i32, i32), (u32, u32)> = HashMap::new();
    for (i, line) in buffer.iter().enumerate() {
        let mut current_num: String = String::from("");
        let mut add_index = (-1, -1);
        for (j, char) in line.chars().enumerate() {
            // println!("Char: {}", char);
            match char.to_string().parse::<u32>() {
                Ok(_) => {
                    for x in -1..2 {
                        for y in -1..2 {
                            let row_index = i as i32 + x;
                            let col_index = j as i32 + y;
                            if x == 0 && y == 0 {
                                continue;
                            }
                            // Checks surrounding cells
                            if row_index >= 0
                                && row_index < buffer.len() as i32
                                && col_index >= 0
                                && col_index < buffer.len() as i32
                            {
                                let symbol = buffer[row_index as usize]
                                    .chars()
                                    .nth(col_index as usize)
                                    .unwrap();
                                if symbol == '*' {
                                    add_index = (row_index, col_index);
                                }
                            }
                        }
                    }
                    current_num.push(char)
                }
                Err(_) => {
                    if !current_num.is_empty() {
                        if add_index.0 > 0 {
                            if let Some(counts) = gear_dict.get_mut(&(add_index.0, add_index.1)) {
                                *counts =
                                    (counts.0 + 1, counts.1 * current_num.parse::<u32>().unwrap());
                            } else {
                                gear_dict.insert(
                                    (add_index.0, add_index.1),
                                    (1, current_num.parse::<u32>().unwrap()),
                                );
                            }
                        }
                        current_num = String::from("");
                        add_index = (-1, -1);
                    }
                }
            }
        }
        if !current_num.is_empty() {
            if add_index.0 > 0 {
                if let Some(counts) = gear_dict.get_mut(&(add_index.0, add_index.1)) {
                    *counts = (counts.0 + 1, counts.1 * current_num.parse::<u32>().unwrap());
                } else {
                    gear_dict.insert(
                        (add_index.0, add_index.1),
                        (1, current_num.parse::<u32>().unwrap()),
                    );
                }
            }
        }
    }
    let mut gear_sum: u32 = 0;
    for count in gear_dict {
        if count.1 .0 == 2 {
            gear_sum += count.1 .1;
        }
    }
    gear_sum
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let buffer: Vec<String> = lines.flatten().map(String::from).collect();
        // let symbols = get_symbols(&buffer);
        println!("{}", part1(&buffer));

        println!("{}", part2(&buffer));
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
