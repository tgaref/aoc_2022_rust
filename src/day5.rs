use aoc_2022_rust::Puzzle;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, sequence::tuple, IResult,
};

#[derive(Debug, Copy, Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct State(Vec<String>);

#[derive(Debug, Clone)]
pub struct Day5 {
    state: State,
    instructions: Vec<Instruction>,
}

fn parse_decimal(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| usize::from_str_radix(s, 10).unwrap())(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            tag("move "),
            parse_decimal,
            tag(" from "),
            parse_decimal,
            tag(" to "),
            parse_decimal,
        )),
        |(_, amount, _, from, _, to)| Instruction { amount, from, to },
    )(input)
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            state: State(Vec::new()),
            instructions: Vec::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.state = State(Vec::new());
        self.instructions = Vec::new()
    }
}

impl Puzzle for Day5 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/5.input");
        let mut read_state = true;
        let mut state = vec![];
        for line in INPUT.lines() {
            if line == "--" {
                read_state = false;
                continue;
            }
            if read_state {
                state.push(line.to_string());
            } else {
                self.instructions.push(parse_instruction(line).unwrap().1)
            }
        }
        self.state = State(state)
    }

    fn part1(&self) -> String {
        let mut state = self.state.clone();
        for instr in &self.instructions {
            let from = &mut state.0[instr.from - 1];
            let l = from.len();
            let chunk = from[l - instr.amount..]
                .to_string()
                .chars()
                .rev()
                .collect::<String>();
            from.truncate(l - instr.amount);
            let to = &mut state.0[instr.to - 1];
            to.push_str(&chunk);
        }
        let mut result = String::new();
        for mut stack in state.0 {
            result.push(stack.pop().unwrap())
        }
        result
    }

    fn part2(&self) -> String {
        let mut state = self.state.clone();
        for instr in &self.instructions {
            let from = &mut state.0[instr.from - 1];
            let l = from.len();
            let chunk = from[l - instr.amount..].to_string();
            from.truncate(l - instr.amount);
            let to = &mut state.0[instr.to - 1];
            to.push_str(&chunk);
        }
        let mut result = String::new();
        for mut stack in state.0 {
            result.push(stack.pop().unwrap())
        }
        result
    }
}
