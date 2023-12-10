#![allow(unused_imports)]
use std::{fs::File, io::{BufReader, BufRead}, error::Error};
use itertools::Itertools;

fn main() {
    println!("** Day 3 **");

    let lines = read_file("input.txt").unwrap();
    let (numbers, symbols) = read_numbers_and_symbols(&lines);

    let part1_answer = part1(&numbers, &symbols);
    println!("Part 1: {part1_answer}");

    let part2_answer = part2(&numbers, &symbols);
    println!("Part 2: {part2_answer}");
}

#[derive(Debug, Clone, Copy)]
struct NumberInfo {
    number: usize,
    location: (usize, usize),
    length: usize,
}

impl NumberInfo {
    fn bounds(&self) -> (usize, usize, usize, usize) {
        let x_min = if self.location.0 > 0 { self.location.0 - 1 } else { 0 };
        let x_max = self.location.0 + self.length;
        let y_min = if self.location.1 > 0 { self.location.1 - 1 } else { 0 };
        let y_max = self.location.1 + 1;
        (x_min, x_max, y_min, y_max)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct SymbolInfo {
    symbol: char,
    location: (usize, usize),
}

fn part1(numbers: &[NumberInfo], symbols: &[SymbolInfo]) -> String {
    let mut sum = 0;
    for number in numbers.iter() {
        // Form a rectangle around the number
        let (x_min, x_max, y_min, y_max) = number.bounds();

        // Check if a symbol is in the rectangle
        let has_symbol = symbols.iter().any(|symbol| {
            symbol.location.0 >= x_min && symbol.location.0 <= x_max &&
            symbol.location.1 >= y_min && symbol.location.1 <= y_max
        });

        // If there is a symbol in the rectangle, add the number to the sum
        if has_symbol {
            sum += number.number;
        }
    }
    sum.to_string()
}

fn part2(numbers: &[NumberInfo], symbols: &[SymbolInfo]) -> String {
    // For each number, find the symbols which are in the rectangle around the number

    let mut symbol_numbers: Vec<Vec<usize>> = symbols.iter().map(|_| Vec::new()).collect();

    for number in numbers.iter() {
        let (x_min, x_max, y_min, y_max) = number.bounds();
        for (i, symbol) in symbols.iter().enumerate() {
            if symbol.location.0 >= x_min && symbol.location.0 <= x_max &&
               symbol.location.1 >= y_min && symbol.location.1 <= y_max {
                symbol_numbers[i].push(number.number);
            }
        }
    }

    let gear_symbols = symbols.iter().enumerate()
        .filter(|(i, symbol)| symbol.symbol == '*' && symbol_numbers[*i].len() == 2)
        .collect_vec();

    let sum = gear_symbols.iter()
        .map(|(i, _)| {
            let (number1, number2) = (symbol_numbers[*i][0], symbol_numbers[*i][1]);
            number1 * number2
        })
        .sum::<usize>();

    sum.to_string()
}

fn read_numbers_and_symbols(lines: &[String]) -> (Vec<NumberInfo>, Vec<SymbolInfo>) {
    let mut numbers: Vec<NumberInfo> = Vec::new();
    let mut symbols: Vec<SymbolInfo> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let digit = (c as u8 - b'0') as usize;
                let mut is_new_number = true;
                if let Some(last_number) = numbers.last() {
                    if last_number.location.1 == y && last_number.location.0 + last_number.length == x {
                        // This is the second or third digit of a number
                        is_new_number = false;
                        numbers.last_mut().unwrap().number *= 10;
                        numbers.last_mut().unwrap().number += digit;
                        numbers.last_mut().unwrap().length += 1;
                    }
                }

                if is_new_number {
                    // This is the first digit of a number
                    numbers.push(NumberInfo {
                        number: digit,
                        location: (x, y),
                        length: 1,
                    });
                }
            } else if c != '.' {
                symbols.push(SymbolInfo {
                    symbol: c,
                    location: (x, y),
                });
            }
        }
    }
    (numbers, symbols)
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
