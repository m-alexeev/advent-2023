use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn str_to_number_vec(input_str: &str) -> HashSet<i32> {
    let split_input = input_str
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| {
            if !s.is_empty() {
                s.trim().parse::<i32>().unwrap()
            } else {
                -1
            }
        })
        .filter(|&num| num >= 0)
        .collect::<HashSet<i32>>();

    split_input
}

fn part1(buffer: &Vec<String>) -> u32 {
    let mut points = 0;
    for line in buffer {
        let split_line = line.split("|").collect::<Vec<&str>>();
        let winning_numbers = str_to_number_vec(split_line[0].split(":").collect::<Vec<&str>>()[1]);
        let player_numbers = str_to_number_vec(split_line[1]);

        let mut num_matches = 0;
        for num in &player_numbers {
            if winning_numbers.contains(num) {
                num_matches += 1;
            }
        }
        if num_matches > 0 {
            points += (2 as u32).pow(num_matches - 1);
        }
    }
    points
}

fn process_cards(buffer: &Vec<String>, bounds: (usize, usize)) -> u32 {
    let mut sum = 0; 
    // for (i, line) in buffer[bounds.0..bounds.1].iter().enumerate() {
    for i in bounds.0..bounds.1{
        let line = &buffer[i];
        let split_line = line.split("|").collect::<Vec<&str>>();
        let winning_numbers = str_to_number_vec(split_line[0].split(":").collect::<Vec<&str>>()[1]);
        let player_numbers = str_to_number_vec(split_line[1]);

        let mut num_matches = 0;
        for num in &player_numbers {
            if winning_numbers.contains(num) {
                num_matches += 1;
            }
        }
        sum += num_matches as u32 + process_cards(&buffer, (i + 1, i + num_matches + 1));
    }
    // return 0 if loop does not iterate (base case)
    return sum 
}

fn part2(buffer: &Vec<String>) -> u32 {
    let scratch_cards = buffer.len() as u32 + process_cards(buffer, (0, buffer.len()));
    scratch_cards
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input") {
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
