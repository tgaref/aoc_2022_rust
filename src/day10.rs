use aoc_2022_rust::Puzzle;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Day10 {
    input: Vec<Instruction>,
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NOOP,
    ADDX(isize),
}

use Instruction::{ADDX, NOOP};

fn parse(line: &str) -> Instruction {
    if let Some((_, n)) = line.split_once(' ') {
        ADDX(n.parse::<isize>().unwrap())
    } else {
        NOOP
    }
}

fn xvalues(instructions: &Vec<Instruction>) -> Vec<isize> {
    let mut values = vec![];
    let mut x = 1isize;
    for instr in instructions {
        match instr {
            NOOP => {
                values.push(x);
            }
            ADDX(n) => {
                values.push(x);
                x += n;
                values.push(x);
            }
        }
    }
    values
}

impl Puzzle for Day10 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/10.input");
        for line in INPUT.lines() {
            self.input.push(parse(line));
        }
    }

    fn part1(&self) -> String {
        let of_interest = [20, 60, 100, 140, 180, 220].iter().collect::<HashSet<_>>();
        let mut strength = 0;
        for (i, x) in xvalues(&self.input).iter().enumerate() {
            if of_interest.contains(&(i + 2)) {
                strength += *x * ((i + 2) as isize);
            }
        }
        format!("{:?}", strength)
    }

    fn part2(&self) -> String {
        let mut pixels = vec!['#'];
        for (i, x) in xvalues(&self.input).iter().enumerate() {
            let pos = (i + 2).rem_euclid(40) as isize;
            if pos == *x || pos == *x + 1 || pos == *x + 2 {
                pixels.push('#');
            } else {
                pixels.push('.');
            }
        }

        let mut code = String::new();
        code.push('\n');
        for i in 0..6 {
            for j in 0..40 {
                code.push_str(&format!("{:}", pixels[i * 40 + j]));
            }
            code.push('\n')
        }
        code
    }
}
