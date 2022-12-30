use aoc_2022_rust::Puzzle;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::cmp::{
    Eq, Ordering,
    Ordering::{Equal, Greater, Less},
    PartialEq,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Elem<T> {
    Num(T),
    List(Vec<Elem<T>>),
}

use Elem::{List, Num};

#[derive(Debug, Clone)]
pub struct Day13 {
    input: Vec<Elem<usize>>,
}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

impl<T: Ord + Copy> Ord for Elem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&List(ref left), &List(ref right)) => {
                for (a, b) in left.iter().zip(right.iter()) {
                    if *a < *b {
                        return Less;
                    } else if *a > *b {
                        return Greater;
                    } else {
                        continue;
                    }
                }
                let alen = left.len();
                let blen = right.len();
                if alen < blen {
                    return Less;
                } else if alen > blen {
                    return Greater;
                } else {
                    return Equal;
                }
            }

            (&Num(left), &Num(right)) => left.cmp(&right),

            (l @ &List(_), &Num(right)) => l.cmp(&List(vec![Num(right)])),

            (&Num(left), r @ &List(_)) => List(vec![Num(left)]).cmp(r),
        }
    }
}

impl<T: Ord + Copy> PartialOrd for Elem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn num_parser(input: &str) -> IResult<&str, Elem<usize>> {
    map(digit1, |s| Num(usize::from_str_radix(s, 10).unwrap()))(input)
}

fn item_parser(input: &str) -> IResult<&str, Elem<usize>> {
    alt((num_parser, list_parser))(input)
}

fn list_parser(input: &str) -> IResult<&str, Elem<usize>> {
    map(
        delimited(
            char('['),
            separated_list0(char(','), item_parser),
            char(']'),
        ),
        |v| List(v),
    )(input)
}

impl Puzzle for Day13 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/13.input");
        for line in INPUT.lines().filter(|s| !s.is_empty()) {
            self.input.push(list_parser(line).unwrap().1);
        }
    }

    fn part1(&self) -> String {
        let mut lines = self.input.iter();
        let mut i = 1;
        let mut count = 0;
        while let Some(line) = lines.next() {
            let left = line;
            let right = lines.next().unwrap();
            if left <= right {
                count += i;
            }
            i += 1;
        }
        format!("{:?}", count)
    }

    fn part2(&self) -> String {
        let mut l = self.input.clone();
        let a = List(vec![List(vec![Num(2)])]);
        let b = List(vec![List(vec![Num(6)])]);
        l.push(a.clone());
        l.push(b.clone());
        l.sort();
        let mut key = 1;
        for (i, item) in l.iter().enumerate() {
            if *item == a || *item == b {
                key *= i + 1;
            }
        }
        format!("{:?}", key)
    }
}
