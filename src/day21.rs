use aoc_2022_rust::Puzzle;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::map,
    sequence::tuple,
    IResult,
};
use num::rational::Rational64;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Day21 {
    input: HashMap<String, Instruction>,
}

impl Day21 {
    pub fn new() -> Day21 {
        Day21 {
            input: HashMap::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = HashMap::new()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

use Operator::{Add, Div, Mul, Sub};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Instruction {
    Val(i64),
    Op(Operator, String, String),
}

use Instruction::{Op, Val};

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    map(
        alt((tag(" + "), tag(" - "), tag(" * "), tag(" / "))),
        |s| match s {
            " + " => Add,
            " - " => Sub,
            " * " => Mul,
            " / " => Div,
            _ => panic!("unrecognized operator"),
        },
    )(input)
}

fn parse_op(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((alpha1, parse_operator, alpha1)),
        |(var1, op, var2)| Op(op, var1.to_string(), var2.to_string()),
    )(input)
}

fn parse_val(input: &str) -> IResult<&str, Instruction> {
    map(complete::i64, |s| Val(s))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_val, parse_op))(input)
}

fn parse_line(input: &str) -> IResult<&str, (String, Instruction)> {
    map(
        tuple((alpha1, tag(": "), parse_instruction)),
        |(var, _, instruction)| (var.to_string(), instruction),
    )(input)
}

impl Puzzle for Day21 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/21.input");
        for line in INPUT.lines() {
            let t = parse_line(line).unwrap().1;
            self.input.insert(t.0, t.1);
        }
    }

    fn part1(&self) -> String {
        let mut vals = self
            .input
            .iter()
            .filter_map(|(name, instr)| match instr {
                Val(n) => Some((name.clone(), *n)),
                _ => None,
            })
            .collect::<HashMap<String, i64>>();
        format!("{:?}", eval(&mut vals, &self.input, "root"))
    }

    fn part2(&self) -> String {
        let mut vals = HashMap::new();
        if let Op(_, ref left, ref right) = self.input["root"] {
            let Expr { a: a1, b: b1 } = compute_expr(&mut vals, &self.input, left);
            let Expr { a: a2, b: b2 } = compute_expr(&mut vals, &self.input, right);
            format!("{:?}", ((b2 - b1) / (a1 - a2)).numer())
        } else {
            panic!("root is not what expected!");
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Expr {
    a: Rational64,
    b: Rational64,
}

fn compute_expr(
    vals: &mut HashMap<String, Expr>,
    input: &HashMap<String, Instruction>,
    name: &str,
) -> Expr {
    if name == "humn" {
        vals.insert(
            name.to_string(),
            Expr {
                a: Rational64::from_integer(1),
                b: Rational64::from_integer(0),
            },
        );
        return vals[name];
    }
    if vals.contains_key(name) {
        return vals[name];
    } else {
        match input[name] {
            Val(n) => {
                vals.insert(
                    name.to_string(),
                    Expr {
                        a: Rational64::from_integer(0),
                        b: Rational64::from_integer(n),
                    },
                );
            }
            Op(op, ref n1, ref n2) => {
                let e1 = compute_expr(vals, input, n1);
                let e2 = compute_expr(vals, input, n2);
                vals.insert(name.to_string(), compute(e1, e2, op));
            }
        }
        return vals[name];
    }
}

fn compute(Expr { a: a1, b: b1 }: Expr, Expr { a: a2, b: b2 }: Expr, op: Operator) -> Expr {
    match op {
        Add => Expr {
            a: a1 + a2,
            b: b1 + b2,
        },
        Sub => Expr {
            a: a1 - a2,
            b: b1 - b2,
        },
        Mul => {
            if a1 * a2 != Rational64::from_integer(0) {
                panic!("Getting a square here!");
            } else {
                Expr {
                    a: (a1 * b2 + a2 * b1),
                    b: b1 * b2,
                }
            }
        }
        Div => {
            if a2 != Rational64::from_integer(0) {
                panic!("Getting a deominator here!");
            } else {
                Expr {
                    a: a1 / b2,
                    b: b1 / b2,
                }
            }
        }
    }
}

fn eval(vals: &mut HashMap<String, i64>, input: &HashMap<String, Instruction>, name: &str) -> i64 {
    if vals.contains_key(name) {
        return vals[name];
    } else {
        match input[name] {
            Op(op, ref a, ref b) => {
                let v = eval(vals, input, a);
                vals.insert(a.clone(), v);
                let u = eval(vals, input, b);
                vals.insert(b.clone(), u);
                match op {
                    Add => return v + u,
                    Sub => return v - u,
                    Mul => return v * u,
                    Div => return v / u,
                }
            }
            Val(n) => return n,
        }
    }
}
