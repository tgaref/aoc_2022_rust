use aoc_2022_rust::Puzzle;
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Debug, Clone)]
pub struct Day6 {
    input: String,
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {
            input: String::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = String::new()
    }
}

impl Puzzle for Day6 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/6.input");
        self.input = INPUT.to_string();
    }

    fn part1(&self) -> String {
        format!("{:?}", solve(&self.input, 4))
    }

    fn part2(&self) -> String {
        format!("{:?}", solve(&self.input, 14))
    }
}

fn solve(input: &str, n: usize) -> usize {
    let mut window: HashMap<char, usize> = HashMap::new();
    let back = input.chars();
    let mut front = input.chars();
    for _ in 0..n {
        let c = front.next().unwrap();
        if let Some(fval) = window.get_mut(&c) {
            *fval += 1;
        } else {
            window.insert(c, 1);
        }
    }
    if window.values().all(|count| *count <= 1) {
        return n;
    }

    for (i, (a, b)) in front.zip(back).enumerate() {
        if let Some(fval) = window.get_mut(&a) {
            *fval += 1;
        } else {
            window.insert(a, 1);
        }
        if let Some(bval) = window.get_mut(&b) {
            *bval -= 1;
        }
        if window.values().all(|count| *count <= 1) {
            return n + i + 1;
        }
    }
    0
}
