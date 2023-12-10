#![allow(unused_imports)]
use std::{fs::File, io::{BufReader, BufRead}, error::Error};
use itertools::Itertools;
use regex::Regex;

fn main() {
    println!("** Day 5 **");

    // Parse the input
    let start_time = std::time::Instant::now();
    let lines = read_file("input.txt").unwrap();
    let almanac = Almanac::from_lines(&lines);
    println!("Parsing took {:?}", start_time.elapsed());

    let start_time = std::time::Instant::now();
    let part1_answer = part1(&almanac);
    println!("Part 1: {part1_answer}");
    println!("Part 1 took {:?}", start_time.elapsed());

    let part2_answer = part2(&almanac);
    let start_time = std::time::Instant::now();
    println!("Part 2: {part2_answer}");
    println!("Part 2 took {:?}", start_time.elapsed());
}

fn part1(almanac: &Almanac) -> String {
    let min_location = almanac.seeds.iter().map(|seed| almanac.map("seed", "location", *seed)).min().unwrap();
    min_location.to_string()
}

fn part2(almanac: &Almanac) -> String {
    let seed_ranges = almanac.seeds.chunks_exact(2).map(|chunk| (chunk[0], chunk[1])).collect::<Vec<(usize, usize)>>();
    let ranges = almanac.map_ranges("seed", "location", &seed_ranges);
    let min_location = ranges.iter().map(|(start, _)| *start).min().unwrap();
    min_location.to_string()
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn from_lines(lines: &[String]) -> Almanac {
        let mut seeds = Vec::new();
        let mut maps = Vec::new();
        let mut map = None;
        for line in lines {
            if line.starts_with("seeds:") {
                seeds = line.split_whitespace().skip(1).map(|s| s.parse::<usize>().unwrap()).collect();
            } else if line.ends_with(" map:") {
                // Line looks like: seed-to-soil map:
                let mut parts = line.split_whitespace();
                let names = parts.nth(0).unwrap().split('-').collect::<Vec<&str>>();
                
                let from = names[0].to_string();
                let to = names[2].to_string();
                map = Some(AlmanacMap { from, to, entries: Vec::new() });
            } else if !line.is_empty() {
                // Line looks like: dst src len
                let parts_usize = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                let dst_start = parts_usize[0];
                let src_start = parts_usize[1];
                let length = parts_usize[2];
                map.as_mut().unwrap().entries.push(AlmanacMapEntry { src_start, dst_start, length });
            } else if let Some(mut map) = map.take() {
                map.normalize();
                maps.push(map);
            }
        }

        if let Some(mut map) = map.take() {
            map.normalize();
            maps.push(map);
        }

        Almanac { seeds, maps }
    }

    fn map(&self, from: &str, to: &str, src: usize) -> usize {
        let mut src = src;
        let mut from = from.to_string();
        while from != to {
            let mut found = false;
            for map in &self.maps {
                if map.from == from {
                    src = map.map(src);
                    from = map.to.clone();
                    found = true;

                    // Normally we would break here, but the next map is probably the next map in the chain
                    // (With the input we have, it always is)
                }
            }

            // Ensure progress was made
            if !found {
                panic!("No map found from {} to {}", from, to);
            }
        }
        src
    }

    fn map_ranges(&self, from: &str, to: &str, ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
        let orig_ranges_len = ranges.len();

        let mut from = from;
        let mut ranges = ranges.to_vec();

        while from != to {
            let mut found = false;
            for map in &self.maps {
                if map.from == from {
                    let new_ranges: Vec<(usize, usize)> = ranges.iter().flat_map(|(start, len)| map.map_range(*start, *len)).collect();
                    assert!(new_ranges.len() >= ranges.len());
                    assert!(new_ranges.iter().all(|(_, len)| *len > 0));
                    ranges = new_ranges;
                    from = &map.to;
                    found = true;

                    // Normally we would break here, but the next map is probably the next map in the chain
                    // (With the input we have, it always is)
                }
            }

            // Ensure progress was made
            if !found {
                panic!("No map found from {} to {}", from, to);
            }
        }

        assert!(ranges.len() >= orig_ranges_len);
        ranges
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct AlmanacMap {
    from: String,
    to: String,
    entries: Vec<AlmanacMapEntry>,
}

impl AlmanacMap {
    fn normalize(&mut self) {
        // Sort the map ranges
        self.entries.sort_by_key(|entry| entry.src_start);

        // Fill in missing ranges
        let mut start = 0;
        let mut new_entries = Vec::new();
        for entry in self.entries.iter() {
            if entry.src_start > start {
                new_entries.push(AlmanacMapEntry { src_start: start, dst_start: start, length: entry.src_start - start });
            }
            new_entries.push(*entry);
            start = entry.src_start + entry.length;
        }
        new_entries.push(AlmanacMapEntry { src_start: start, dst_start: start, length: usize::MAX - start });

        self.entries = new_entries;
    }

    fn map(&self, src: usize) -> usize {
        for entry in &self.entries {
            if let Some(dst) = entry.map(src) {
                return dst;
            }
        }
        src
    }

    fn map_range(&self, start: usize, len: usize) -> Vec<(usize, usize)> {
        assert!(self.entries.first().unwrap().begin() == 0 && self.entries.last().unwrap().end() == usize::MAX);

        let initial_start = start;
        let mut start = start;
        let mut len = len;
        let mut ranges = Vec::new();

        for entry in self.entries.iter().filter(|entry| entry.end() > initial_start) {
            if len == 0 {
                break;
            }

            let start_offset = start - entry.src_start;
            let entry_used = entry.length - start_offset;
            let dst_start = entry.map(start).unwrap();
            let dst_len = entry_used.min(len);
            assert!(dst_len > 0);
            ranges.push((dst_start, dst_len));

            // Update the range
            start = entry.src_start + entry.length;
            len -= dst_len;
        }

        ranges
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct AlmanacMapEntry {
    dst_start: usize,
    src_start: usize,
    length: usize,
}

impl AlmanacMapEntry {
    fn map(&self, src: usize) -> Option<usize> {
        if src < self.src_start || src >= self.src_start + self.length {
            return None;
        }
        let offset = src - self.src_start;
        Some(self.dst_start + offset)
    }

    fn begin(&self) -> usize {
        self.src_start
    }

    // One past the last index
    fn end(&self) -> usize {
        self.src_start + self.length
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
