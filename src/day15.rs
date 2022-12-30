use aoc_2022_rust::Puzzle;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::many0_count,
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Day15 {
    input: Scan,
}

type Scan = HashMap<Position, Position>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Day15 {
    pub fn new() -> Day15 {
        Day15 {
            input: HashMap::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = HashMap::new()
    }
}

fn parse_int(input: &str) -> IResult<&str, i32> {
    map(tuple((many0_count(char('-')), digit1)), |(sign, s)| {
        let n = i32::from_str_radix(s, 10).unwrap();
        if sign % 2 == 0 {
            n
        } else {
            -n
        }
    })(input)
}

fn parse_position(input: &str) -> IResult<&str, Position> {
    map(
        tuple((tag("x="), parse_int, tag(", y="), parse_int)),
        |(_, x, _, y)| Position { x, y },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (Position, Position)> {
    map(
        tuple((
            tag("Sensor at "),
            parse_position,
            tag(": closest beacon is at "),
            parse_position,
        )),
        |(_, p1, _, p2)| (p1, p2),
    )(input)
}

fn distance(p1: Position, p2: Position) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn radii_of_sensors(scan: &Scan) -> HashMap<Position, i32> {
    let mut radii = HashMap::new();
    for (s, b) in scan {
        radii.insert(*s, distance(*s, *b));
    }
    radii
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Type {
    Open,
    Close,
}

use Type::{Close, Open};

fn uncovered_on_line(xs: Vec<(i32, Type)>, end: i32) -> Option<u128> {
    let mut open_count = 0;
    let mut prev_close = -1;
    if xs[xs.len() - 1].0 < end {
        return Some((xs[xs.len() - 1].0 + 1) as u128);
    }
    for (x, t) in xs {
        match t {
            Open => {
                if open_count == 0 && prev_close < x - 1 {
                    return Some((x - 1) as u128);
                }
                open_count += 1;
            }
            Close => {
                open_count -= 1;
                prev_close = x;
            }
        }
    }
    None
}

fn covered_on_line(xs: Vec<(i32, Type)>) -> i32 {
    let mut open_count = 0;
    let mut prev_open = -1;
    let mut covered = 0;
    for (x, t) in xs {
        match t {
            Open => {
                if open_count == 0 {
                    prev_open = x;
                }
                open_count += 1;
                if open_count == 0 {
                    prev_open = x;
                }
            }
            Close => {
                open_count -= 1;
                if open_count == 0 {
                    covered += x - prev_open + 1;
                }
            }
        }
    }
    covered
}

impl Puzzle for Day15 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/15.input");
        for line in INPUT.lines() {
            let (sensor, beacon) = parse_line(line).unwrap().1;
            self.input.insert(sensor, beacon);
        }
    }

    fn part1(&self) -> String {
        let line_y = 2000000i32;
        let radii = radii_of_sensors(&self.input);
        let mut xs = Vec::new();
        for (s, d) in &radii {
            let y_dist = (line_y - s.y).abs();
            if *d >= y_dist {
                let left = s.x - (d - y_dist);
                let right = s.x + (d - y_dist);
                if self.input[s].x != left {
                    xs.push((left, Open));
                } else {
                    xs.push((left + 1, Open));
                }
                if self.input[s].x != right {
                    xs.push((right, Close));
                } else {
                    xs.push((right - 1, Close));
                }
            }
        }
        xs.sort();

        format!("{:?}", covered_on_line(xs))
    }

    fn part2(&self) -> String {
        let radii = radii_of_sensors(&self.input);
        let mut y = 4000001;
        let mut some_x = None;
        while let None = some_x {
            y -= 1;
            let mut xs = Vec::new();
            for (s, d) in &radii {
                let y_dist = (y - s.y).abs();
                if *d >= y_dist {
                    let left = (s.x - (d - y_dist)).max(0);
                    let right = (s.x + (d - y_dist)).max(0);
                    xs.push((left.min(4000000), Open));
                    xs.push((right.min(4000000), Close));
                }
            }
            xs.sort();
            some_x = uncovered_on_line(xs, 4000000);
        }

        format!("{:?}", some_x.unwrap() * 4000000 + y as u128)
    }
}
