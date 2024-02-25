use aoc_2022_rust::Puzzle;
use nom::{
    character::complete::{char, digit1},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    a: u64,
    b: u64,
}

impl Interval {
    fn contains(&self, other: &Self) -> bool {
        (self.a <= other.a) && (other.b <= self.b)
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.b >= other.a && self.b <= other.b) || (other.b >= self.a && other.b <= self.b)
    }
}

#[derive(Debug, Clone)]
pub struct Day4 {
    input: Vec<(Interval, Interval)>,
}

fn parse_decimal(input: &str) -> IResult<&str, u64> {
    map(digit1, |s: &str| u64::from_str_radix(s, 10).unwrap())(input)
}

fn parse_interval(input: &str) -> IResult<&str, Interval> {
    map(
        separated_pair(parse_decimal, char('-'), parse_decimal),
        |(a, b)| Interval { a, b },
    )(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Interval, Interval)> {
    separated_pair(parse_interval, char(','), parse_interval)(input)
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

impl Puzzle for Day4 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/4.input");
        for line in INPUT.lines() {
            self.input.push(parse_pair(line).unwrap().1);
        }
    }

    fn part1(&self) -> String {
        self.input
            .iter()
            .filter(|(int1, int2)| int1.contains(int2) || int2.contains(int1))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.input
            .iter()
            .filter(|(int1, int2)| int1.overlaps(int2))
            .count()
            .to_string()
    }
}
