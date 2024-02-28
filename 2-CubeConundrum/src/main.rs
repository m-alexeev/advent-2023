use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]

enum Color {
    Red,
    Green,
    Blue,
}

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

fn part1(buffer: &Vec<String>) -> u32 {
    let mut game_id_sum: u32 = 0;
    for line in buffer {
        let game_str = line.split(":").collect::<Vec<&str>>()[0];
        let color_str = line.split(":").collect::<Vec<&str>>()[1]
            .strip_prefix(" ")
            .unwrap();
        let game_id = game_str.split(" ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();

        let color_combos: Vec<&str> = color_str.split(";").collect::<Vec<&str>>();
        let mut add = true;

        for combo in color_combos {
            let combo_strs = combo.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
            let color_counts: Vec<(u32, Color)> = combo_strs
                .iter()
                .map(|x| {
                    let mapping: Vec<&str> = x.split(" ").collect();
                    (
                        mapping[0].parse::<u32>().unwrap(),
                        match mapping[1] {
                            "green" => Color::Green,
                            "blue" => Color::Blue,
                            "red" => Color::Red,
                            _ => Color::Red,
                        },
                    )
                })
                .collect();
            for (count, color) in color_counts {
                if color == Color::Blue {
                    if count > BLUE_CUBES {
                        add = false;
                        break;
                    }
                }
                if color == Color::Green {
                    if count > GREEN_CUBES {
                        add = false;
                        break;
                    }
                }
                if color == Color::Red {
                    if count > RED_CUBES {
                        add = false;
                        break;
                    }
                }
            }
            if !add {
                break;
            }
        }
        if add {
            game_id_sum = game_id_sum + game_id;
        }
    }
    game_id_sum
}

fn part2(buffer: &Vec<String>) -> u32 {
    let mut game_power_sum: u32 = 0;
    let mut min_cubes: Vec<u32> = vec![0,0,0];
    for line in buffer {
        let color_str = line.split(":").collect::<Vec<&str>>()[1]
            .strip_prefix(" ")
            .unwrap();

        let color_combos: Vec<&str> = color_str.split(";").collect::<Vec<&str>>();

        for combo in color_combos {
            let combo_strs = combo.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
            let color_counts: Vec<(u32, Color)> = combo_strs
                .iter()
                .map(|x| {
                    let mapping: Vec<&str> = x.split(" ").collect();
                    (
                        mapping[0].parse::<u32>().unwrap(),
                        match mapping[1] {
                            "green" => Color::Green,
                            "blue" => Color::Blue,
                            "red" => Color::Red,
                            _ => Color::Red,
                        },
                    )
                })
                .collect();
            for (count, color) in color_counts {
                if color == Color::Red{
                    if count > min_cubes[0]{
                        min_cubes[0] = count;
                    }
                }
                if color == Color::Green{
                    if count > min_cubes[1]{
                        min_cubes[1] = count;
                    }
                }
                if color == Color::Blue{
                    if count > min_cubes[2]{
                        min_cubes[2] = count;
                    }
                }
            }

        }
        game_power_sum = game_power_sum + (min_cubes[0] * min_cubes[1] * min_cubes[2]);

        min_cubes = vec![0,0,0];
    }
    game_power_sum 
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
