use aoc_2022_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day1 {
    input: Vec<usize>,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

impl Puzzle for Day1 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/1.input");
        let mut elf = vec![];
        for line in INPUT.lines() {
            if line.is_empty() {
                self.input.push(elf.iter().sum::<usize>());
                elf = vec![]
            } else {
                elf.push(line.parse().unwrap())
            }
        }
        self.input.push(elf.iter().sum::<usize>());
        self.input.sort_by(|a, b| b.cmp(a));
    }

    fn part1(&self) -> String {
        format!("{:}", self.input.iter().next().unwrap())
    }

    fn part2(&self) -> String {
        format!("{:}", self.input.iter().take(3).sum::<usize>())
    }
}
