use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn part1(buffer: &mut io::Lines<io::BufReader<File>>) -> u32 {
    let mut running_sum: u32 = 0;
    for line in buffer.flatten(){
        // Find the leftmost numeric character in the line
        let left = line.chars().find(|ch| ch.is_numeric());

        // Find the rightmost numeric character in the line
        let right = line.chars().rev().find(|ch| ch.is_numeric());

        if let (Some(left_num), Some(right_num)) = (left, right) {
            let left_value = left_num.to_digit(10).unwrap() * 10;
            let right_value = right_num.to_digit(10).unwrap();
            running_sum = running_sum + left_value + right_value;
        } else {
            println!("No numeric characters found in the line.");
        }
    }
    return  running_sum;
}

fn part2(buffer: &mut io::Lines<io::BufReader<File>>) -> u32 {
    let running_sum: u32 = 0;

    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in buffer.flatten(){
        let mut parsed_digits: Vec<u32> = vec![];
        for digit in digits.iter(){
        }
    }
    return running_sum;
}


fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(mut lines) = read_lines("./sample") {
        // Consumes the iterator, returns an (Optional) String
        println!("{}", part1(&mut lines));
        println!("{}", part2(&mut lines));
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}