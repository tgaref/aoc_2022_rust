use aoc_2022_rust::Puzzle;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::many1,
    sequence::tuple,
    IResult,
};
use num::integer::lcm;

#[derive(Debug, Clone)]
pub struct Day11 {
    input: Vec<Monkey>,
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    iftrue: usize,
    iffalse: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            items: Vec::new(),
            operation: MulOld,
            test: 1,
            iftrue: 0,
            iffalse: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Mul(u64),
    Add(u64),
    MulOld,
    AddOld,
}

use Operation::{Add, AddOld, Mul, MulOld};

impl Operation {
    fn parse(input: &str) -> IResult<&str, Operation> {
        let tp = tuple((
            tag("  Operation: new = old "),
            alt((tag("+ "), tag("* "))),
            alt((tag("old"), digit1)),
        ));
        map(tp, |(_, op, rhs)| match op {
            "+ " => {
                if rhs == "old" {
                    AddOld
                } else {
                    Add(u64::from_str_radix(rhs, 10).unwrap())
                }
            }
            "* " => {
                if rhs == "old" {
                    MulOld
                } else {
                    Mul(u64::from_str_radix(rhs, 10).unwrap())
                }
            }
            _ => panic!("unrecognized operation"),
        })(input)
    }
}

fn parse_id(input: &str) -> IResult<&str, usize> {
    let id = tuple((tag("Monkey "), digit1, char(':')));
    map(id, |(_, i, _)| usize::from_str_radix(i, 10).unwrap())(input)
}

fn parse_items(input: &str) -> IResult<&str, Vec<u64>> {
    let p = tuple((tag("  Starting items: "), many1(alt((digit1, tag(", "))))));
    map(p, |(_, v)| {
        v.into_iter()
            .filter(|&s| s != ", ")
            .map(|s| u64::from_str_radix(s, 10).unwrap())
            .collect::<Vec<_>>()
    })(input)
}

fn parse_test(input: &str) -> IResult<&str, u64> {
    let testp = tuple((tag("  Test: divisible by "), digit1));
    map(testp, |(_, s)| u64::from_str_radix(s, 10).unwrap())(input)
}

fn parse_target(input: &str) -> IResult<&str, usize> {
    let p = tuple((
        alt((
            tag("    If true: throw to monkey "),
            tag("    If false: throw to monkey "),
        )),
        digit1,
    ));
    map(p, |(_, s)| usize::from_str_radix(s, 10).unwrap())(input)
}

impl Puzzle for Day11 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/11.input");
        let mut monkey = Monkey::new();
        for (i, line) in INPUT.lines().enumerate() {
            match i.rem_euclid(7) {
                0 => monkey.id = parse_id(line).unwrap().1,
                1 => monkey.items = parse_items(line).unwrap().1,
                2 => monkey.operation = Operation::parse(line).unwrap().1,
                3 => monkey.test = parse_test(line).unwrap().1,
                4 => monkey.iftrue = parse_target(line).unwrap().1,
                5 => monkey.iffalse = parse_target(line).unwrap().1,
                6 => self.input.push(monkey.clone()),
                _ => panic!("This is not possible!"),
            }
        }
    }

    fn part1(&self) -> String {
        let monkeys = self.input.clone();
        simulate(monkeys, 20, None, Some(3))
    }

    fn part2(&self) -> String {
        let monkeys = self.input.clone();
        let n = monkeys.iter().fold(1, |acc, m| lcm(acc, m.test));
        simulate(monkeys, 10000, Some(n), None)
    }
}

fn simulate(
    mut monkeys: Vec<Monkey>,
    steps: usize,
    modulus: Option<u64>,
    reduce: Option<u64>,
) -> String {
    let mut inspections: Vec<u64> = Vec::with_capacity(monkeys.len());
    for _ in 0..monkeys.len() {
        inspections.push(0)
    }
    let mut new_inspections: Vec<u64>;
    for _ in 0..steps {
        (monkeys, new_inspections) = round(monkeys, modulus, reduce);
        inspections = inspections
            .iter()
            .zip(new_inspections.into_iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<_>>();
    }
    inspections.sort_by(|a, b| b.cmp(a));
    format!("{:?}", inspections[0] * inspections[1])
}

fn update_worry_level(wl: u64, op: Operation, modulus: Option<u64>, reduce: Option<u64>) -> u64 {
    let mut val = match op {
        Mul(n) => wl * n,
        Add(n) => wl + n,
        MulOld => wl * wl,
        AddOld => wl + wl,
    };
    if let Some(n) = modulus {
        val = val.rem_euclid(n)
    };
    if let Some(m) = reduce {
        val = val.div_euclid(m)
    };
    val
}

fn round(
    mut monkeys: Vec<Monkey>,
    modulus: Option<u64>,
    reduce: Option<u64>,
) -> (Vec<Monkey>, Vec<u64>) {
    let mut inspections = Vec::<u64>::new();
    let mut new_monkeys;
    for n in 0..monkeys.len() {
        new_monkeys = monkeys.clone();
        let monkey = &monkeys[n];
        inspections.push(monkey.items.len() as u64);
        for worry_level in &monkey.items {
            let new_worry_level =
                update_worry_level(*worry_level, monkey.operation, modulus, reduce);
            if new_worry_level.rem_euclid(monkey.test) == 0 {
                new_monkeys[monkey.iftrue].items.push(new_worry_level)
            } else {
                new_monkeys[monkey.iffalse].items.push(new_worry_level)
            };
        }
        new_monkeys[n].items = Vec::new();
        monkeys = new_monkeys;
    }
    (monkeys, inspections)
}
