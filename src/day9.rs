use aoc_2022_rust::Puzzle;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Day9 {
    input: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    dir: (isize, isize),
    steps: isize,
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

type Position = (isize, isize);

const LEFT: Position = (-1, 0);
const RIGHT: Position = (1, 0);
const UP: Position = (0, 1);
const DOWN: Position = (0, -1);

fn parse(line: &str) -> Instruction {
    let steps = line[2..].parse::<isize>().unwrap();
    match line.chars().next().unwrap() {
        'L' => Instruction { dir: LEFT, steps },
        'R' => Instruction { dir: RIGHT, steps },
        'U' => Instruction { dir: UP, steps },
        'D' => Instruction { dir: DOWN, steps },
        _ => panic!("Should not happen"),
    }
}

fn is_close(head: Position, tail: Position) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

fn follow(head: Position, tail: Position) -> Position {
    if !is_close(head, tail) {
        let dirx = (head.0 - tail.0).signum();
        let diry = (head.1 - tail.1).signum();

        (tail.0 + dirx, tail.1 + diry)
    } else {
        tail
    }
}

fn simulate(instructions: &Vec<Instruction>, n: usize) -> usize {
    let mut visited: HashSet<Position> = HashSet::from([(0, 0)]);
    let mut knots = Vec::with_capacity(n);
    let mut newknots = Vec::with_capacity(n);
    for _ in 0..n {
        knots.push((0, 0));
        newknots.push((0, 0));
    }
    for instr in instructions {
        for _ in 0..instr.steps {
            newknots[0] = (knots[0].0 + instr.dir.0, knots[0].1 + instr.dir.1);
            for i in 1..n {
                newknots[i] = follow(newknots[i - 1], knots[i]);
            }
            visited.insert(newknots[n - 1]);
            std::mem::swap(&mut knots, &mut newknots);
        }
    }
    visited.len()
}

impl Puzzle for Day9 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/9.input");
        for line in INPUT.lines() {
            self.input.push(parse(line));
        }
    }

    fn part1(&self) -> String {
        format!("{:}", simulate(&self.input, 2))
    }

    fn part2(&self) -> String {
        format!("{:}", simulate(&self.input, 10))
    }
}
