use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(buffer: &Vec<String>) -> u32 {
    for line in buffer{
        let game_str= line.split(":").collect::<Vec<&str>>()[0];
        let game_id = game_str.split(" ").collect::<Vec<&str>>()[1];
            
        println!("{:?}", game_id.parse::<u32>().unwrap());
    }
    0
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
