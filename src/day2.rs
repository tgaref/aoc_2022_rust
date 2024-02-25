use aoc_2022_rust::Puzzle;
use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, Clone)]
pub struct Day2 {
    input: Vec<(Hand, Hand)>,
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn from_str(s: &str) -> Hand {
        if s == "A" || s == "X" {
            Hand::Rock
        } else if s == "B" || s == "Y" {
            Hand::Paper
        } else if s == "C" || s == "Z" {
            Hand::Scissors
        } else {
            panic!("Unrecognized symbol!")
        }
    }

    fn credit(self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn wins_to(self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn looses_to(self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Hand::Rock => {
                if *other == Hand::Paper {
                    Some(Ordering::Less)
                } else if *other == Hand::Scissors {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            Hand::Paper => {
                if *other == Hand::Scissors {
                    Some(Ordering::Less)
                } else if *other == Hand::Rock {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            Hand::Scissors => {
                if *other == Hand::Rock {
                    Some(Ordering::Less)
                } else if *other == Hand::Paper {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
        }
    }
}

impl Puzzle for Day2 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/2.input");
        for line in INPUT.lines() {
            let mut line = line.split(' ');
            let you = Hand::from_str(line.next().unwrap());
            let me = Hand::from_str(line.next().unwrap());
            self.input.push((you, me));
        }
    }

    fn part1(&self) -> String {
        let points = self
            .input
            .iter()
            .fold(0, |acc, (you, me)| acc + me.credit() + gain(*you, *me));
        format!("{:}", points)
    }

    fn part2(&self) -> String {
        let points = self.input.iter().fold(0, |acc, (you, end)| {
            let me = match end {
                Hand::Rock => you.wins_to(),       // I loose
                Hand::Paper => *you,               // Draw
                Hand::Scissors => you.looses_to(), // I win
            };
            acc + me.credit() + gain(*you, me)
        });
        format!("{:}", points)
    }
}

fn gain(you: Hand, me: Hand) -> usize {
    if you > me {
        0
    } else if you < me {
        6
    } else {
        3
    }
}
