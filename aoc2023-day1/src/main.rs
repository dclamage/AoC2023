use std::{fs::File, io::{BufReader, BufRead}, error::Error, vec};

fn main() {
    println!("** Day 1 **");

    let lines = read_file("input.txt").unwrap();
    let part1_answer = part1(&lines);
    println!("Part 1: {part1_answer}");

    let part2_answer = part2(&lines);
    println!("Part 2: {part2_answer}");
}

fn parse_first_last_number(line: &str) -> (usize, usize) {
    let mut first_number = usize::MAX;
    let mut last_number = usize::MAX;
    for c in line.chars() {
        if c.is_ascii_digit() {
            if first_number == usize::MAX {
                first_number = c.to_digit(10).unwrap() as usize;
            }
            last_number = c.to_digit(10).unwrap() as usize;
        }
    }
    (first_number, last_number)
}

fn part1(lines: &[String]) -> String {
    let mut sum = 0;
    for line in lines.iter() {
        let (first_number, last_number) = 
            parse_first_last_number(line);
        sum += first_number * 10 + last_number;
    }
    sum.to_string()
}

fn find_first_of_string_list(line: &str, strings: &[&str]) -> Option<(usize, usize)> {
    let mut first_index = usize::MAX;
    let mut first_value = usize::MAX;
    for (value, string) in strings.iter().enumerate() {
        if let Some(i) = line.find(string) {
            if i < first_index {
                first_index = i;
                first_value = value + 1;
            }
        }
    }
    if first_index == usize::MAX {
        None
    } else {
        Some((first_index, first_value))
    }
}

fn find_last_of_string_list(line: &str, strings: &[&str]) -> Option<(usize, usize)> {
    let mut last_index = usize::MAX;
    let mut last_value = usize::MAX;
    for (value, string) in strings.iter().enumerate() {
        if let Some(i) = line.rfind(string) {
            if last_index == usize::MAX || i > last_index {
                last_index = i;
                last_value = value + 1;
            }
        }
    }
    if last_index == usize::MAX {
        None
    } else {
        Some((last_index, last_value))
    }
}

fn parse_first_last_number_pt2(line: &str) -> (usize, usize) {
    let digit_strings = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut first_digit_value = usize::MAX;
    let mut first_digit_index = usize::MAX;
    if let Some((i, v)) = find_first_of_string_list(line, &digit_strings) {
        first_digit_index = i;
        first_digit_value = v;
    }
    
    let digit_value_strings = vec!("1", "2", "3", "4", "5", "6", "7", "8", "9");
    if let Some((i, v)) = find_first_of_string_list(line, &digit_value_strings) {
        if i < first_digit_index {
            first_digit_value = v;
        }
    }

    assert!(first_digit_value != usize::MAX);

    let mut last_digit_value = usize::MAX;
    let mut last_digit_index = usize::MAX;
    if let Some((i, v)) = find_last_of_string_list(line, &digit_strings) {
        last_digit_index = i;
        last_digit_value = v;
    }

    if let Some((i, v)) = find_last_of_string_list(line, &digit_value_strings) {
        if last_digit_index == usize::MAX || i > last_digit_index {
            last_digit_value = v;
        }
    }
    assert!(last_digit_value != usize::MAX);

    (first_digit_value, last_digit_value)
}

fn part2(lines: &[String]) -> String {
    let mut sum = 0;
        for line in lines.iter() {
        let (first_number, last_number) = 
            parse_first_last_number_pt2(line);
        sum += first_number * 10 + last_number;
    }
    sum.to_string()
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
