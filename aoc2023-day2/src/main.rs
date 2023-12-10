use std::{fs::File, io::{BufReader, BufRead}, error::Error};
use itertools::Itertools;

fn main() {
    println!("** Day 2 **");
    
    let lines = read_file("input.txt").unwrap().iter().map(|line| parse_line(line)).collect_vec();
    let part1_answer = part1(&lines);
    println!("Part 1: {part1_answer}");

    let part2_answer = part2(&lines);
    println!("Part 2: {part2_answer}");
}

fn part1(lines: &[LineInfo]) -> String {
    // Only 12 red cubes, 13 green cubes, and 14 blue cubes
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    // Sum the line ids for lines which do not have too many cubes of a given color
    let mut sum = 0;
    for line in lines.iter() {
        let mut is_valid = true;
        for dice in line.dice.iter() {
            for die in dice.iter() {
                if die.color == "red" && die.count > MAX_RED {
                    is_valid = false;
                    break;
                }
                if die.color == "green" && die.count > MAX_GREEN {
                    is_valid = false;
                    break;
                }
                if die.color == "blue" && die.count > MAX_BLUE {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                break;
            }
        }

        if is_valid {
            sum += line.id;
        }
    }
    
    sum.to_string()
}

fn part2(lines: &[LineInfo]) -> String {
    let mut sum = 0;

    for line in lines.iter() {
        // Determine the maximum number of dice of each color
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for dice in line.dice.iter() {
            for die in dice.iter() {
                if die.color == "red" && die.count > max_red {
                    max_red = die.count;
                }
                if die.color == "green" && die.count > max_green {
                    max_green = die.count;
                }
                if die.color == "blue" && die.count > max_blue {
                    max_blue = die.count;
                }
            }
        }

        let product = max_red * max_green * max_blue;
        sum += product;
    }
    
    sum.to_string()
}

struct DiceInfo {
    count: usize,
    color: String,
}

struct LineInfo {
    id: usize,
    dice: Vec<Vec<DiceInfo>>,
}

fn parse_line(line: &str) -> LineInfo {
    // Split on the colon
    let (label, info) = line.split(": ").collect_tuple::<(&str, &str)>().unwrap();

    // Parse the label of the form "Game #"
    let id = label.split(' ').collect_tuple::<(&str, &str)>().unwrap().1.parse::<usize>().unwrap();

    // Split the info on semicolons
    let dice_strings = info.split("; ");
    
    let mut dice = Vec::new();
    for dice_string in dice_strings {
        let mut dice_info = Vec::new();
        let dice_strings = dice_string.split(", ");
        for dice_string in dice_strings {
            let (count, color) = dice_string.split(' ').collect_tuple().unwrap();
            dice_info.push(DiceInfo {
                count: count.parse::<usize>().unwrap(),
                color: color.to_string().trim().to_ascii_lowercase(),
            });
        }
        dice.push(dice_info);
    }

    LineInfo {
        id,
        dice,
    }
}

fn read_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}
