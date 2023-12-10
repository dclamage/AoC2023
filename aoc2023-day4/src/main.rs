#![allow(unused_imports)]
use std::{fs::File, io::{BufReader, BufRead}, error::Error};
use itertools::Itertools;
use regex::Regex;

fn main() {
    println!("** Day 4 **");

    let lines = read_file("input.txt").unwrap();
    let tickets = lines.iter().map(|line| Ticket::new(line)).collect::<Vec<Ticket>>();

    let part1_answer = part1(&tickets);
    println!("Part 1: {part1_answer}");

    let part2_answer = part2(&tickets);
    println!("Part 2: {part2_answer}");
}

fn part1(tickets: &[Ticket]) -> String {
    tickets.iter().map(|ticket| ticket.score()).sum::<usize>().to_string()
}

fn part2(tickets: &[Ticket]) -> String {
    // Make a copy of the tickets
    let mut tickets = tickets.to_vec();
    
    // For each ticket, get the winning count and increase the copies of count number of tickets after it
    for i in 0..tickets.len() {
        let winning_count = tickets[i].winning_count();
        for j in 1..=winning_count {
            tickets[i + j].copies += tickets[i].copies;
        }
    }

    // Count the total number of tickets
    let total_tickets = tickets.iter().map(|ticket| ticket.copies).sum::<usize>();
    total_tickets.to_string()
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Ticket {
    id: usize,
    winning_numbers: Vec<usize>,
    have_numbers: Vec<usize>,
    copies: usize,
}

#[allow(dead_code)]
impl Ticket {
    fn new(line: &str) -> Ticket {
        // Parse a line like this:
        // Card   1: 98 16 95 90 53 33 43  7 46 45 | 85 15 78 57 34 10 46 90 33 13  8 54  4 37 25 63 55 41  7 82 69 16 30 76  2

        // Use regex to parse the line
        let re = Regex::new(r"Card\s+(\d+):\s+((\d+\s*)+)\|\s+((\d+\s*)+)").unwrap();
        let captures = re.captures(line).unwrap();

        // Get the card id
        let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

        // Get the winning numbers
        let mut winning_numbers = captures.get(2).unwrap().as_str().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        winning_numbers.sort();

        // Get the numbers on the card
        let mut have_numbers = captures.get(4).unwrap().as_str().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        have_numbers.sort();

        // Create the ticket
        Ticket {
            id,
            winning_numbers,
            have_numbers,
            copies: 1,
        }
    }

    fn winning_count(&self) -> usize {
        let mut i = 0;
        let mut j = 0;
        let mut winning_count = 0;

        while i < self.have_numbers.len() && j < self.winning_numbers.len() {
            match self.have_numbers[i].cmp(&self.winning_numbers[j]) {
                std::cmp::Ordering::Less => {
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    j += 1;
                }
                std::cmp::Ordering::Equal => {
                    winning_count += 1;
                    i += 1;
                    j += 1;
                }
            }
        }

        winning_count
    }

    fn score(&self) -> usize {
        let winning_count = self.winning_count();
        if winning_count == 0 {
            0
        } else {
            1 << (winning_count - 1)
        }
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
