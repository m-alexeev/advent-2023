use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;

fn part1(buffer: &Vec<String>) -> u32 {
    let mut part_sum = 0;
    for (i, line) in buffer.iter().enumerate() {
        let mut current_num: String = String::from("");
        let mut can_add = true;
        for (j, char) in line.chars().enumerate() {
            match char.to_string().parse::<u32>() {
                Ok(_) => {
                    // for x in -1..2 {
                        // for y in -1..2 {
                            // if x != 0 && y != 0 {
                    // if i > 0 && i < buffer.len() - 1 && j > 0 && j < line.len() - 1 {
                        // do the check
                        println!("{}", buffer[i+ 1].chars().nth(j).unwrap());
                    // }
                            // }
                        // }
                    // }
                    // check each char if can be added
                    // Check on grid
                    // 0 , 0 , 0
                    // 0 , x , 0
                    // 0 , 0 , 0
                    current_num.push(char)
                }
                Err(_) => {
                    if !current_num.is_empty() {
                        part_sum = part_sum + current_num.parse::<u32>().unwrap();
                        println!("{:?}", current_num.parse::<u32>().unwrap());
                        current_num = String::from("");
                        can_add = true;
                    }
                }
            }
        }
    }

    part_sum
}

fn part2(_buffer: &Vec<String>) -> u32 {
    0
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./sample-1") {
        // Consumes the iterator, returns an (Optional) String
        let buffer: Vec<String> = lines.flatten().map(String::from).collect();
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
