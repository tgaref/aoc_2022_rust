use aoc_2022_rust::Puzzle;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Day3 {
    input: Vec<String>,
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

impl Puzzle for Day3 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/3.input");
        for line in INPUT.lines() {
            self.input.push(line.to_string())
        }
    }

    fn part1(&self) -> String {
        let count = self.input.iter().map(|s| priority(s)).sum::<usize>();

        format!("{:}", count)
    }

    fn part2(&self) -> String {
        let count = self
            .input
            .chunks(3)
            .map(|chunk| badge(chunk))
            .sum::<usize>();
        format!("{:}", count)
    }
}

fn value(c: char) -> usize {
    if c.is_ascii_lowercase() {
        (c as usize) - ('a' as usize) + 1
    } else {
        (c as usize) - ('A' as usize) + 27
    }
}

fn priority(rucksack: &str) -> usize {
    let n = rucksack.len();
    let compartment1 = &rucksack[..n / 2];
    let compartment2 = rucksack[n / 2..].chars().collect::<HashSet<_>>();
    for c in compartment1.chars() {
        if compartment2.contains(&c) {
            return value(c);
        }
    }
    0
}

fn badge(chunk: &[String]) -> usize {
    let ruck1 = chunk[0].chars().collect::<HashSet<_>>();
    let ruck2 = chunk[1].chars().collect::<HashSet<_>>();
    for c in chunk[2].chars() {
        if ruck1.contains(&c) && ruck2.contains(&c) {
            return value(c);
        }
    }
    0
}
