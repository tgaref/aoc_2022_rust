use aoc_2022_rust::Puzzle;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::ops::Add;

type Map = HashMap<Position, char>;

#[derive(Debug, Clone)]
pub struct Day14 {
    input: Map,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Add<(isize, isize)> for Position {
    type Output = Self;

    fn add(self, other: (isize, isize)) -> Self {
        Position {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        } else if self.y > other.y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 {
            input: HashMap::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = HashMap::new()
    }
}

fn parse_position(input: &str) -> IResult<&str, Position> {
    map(separated_pair(digit1, char(','), digit1), |(x, y)| {
        Position {
            x: isize::from_str_radix(x, 10).unwrap(),
            y: isize::from_str_radix(y, 10).unwrap(),
        }
    })(input)
}

fn parse_path(input: &str) -> IResult<&str, Vec<Position>> {
    separated_list1(tag(" -> "), parse_position)(input)
}

fn get_positions(
    Position { x: x1, y: y1 }: Position,
    Position { x: x2, y: y2 }: Position,
) -> Vec<Position> {
    let mut path = Vec::new();
    let (xmin, xmax) = (x1.min(x2), x1.max(x2));
    let (ymin, ymax) = (y1.min(y2), y1.max(y2));
    for x in xmin..=xmax {
        for y in ymin..=ymax {
            path.push(Position { x, y });
        }
    }
    path
}

const DOWN: (isize, isize) = (0, 1);
const LEFTDOWN: (isize, isize) = (-1, 1);
const RIGHTDOWN: (isize, isize) = (1, 1);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    RD,
    LD,
    D,
}

use Direction::{D, LD, RD};

fn next_dir(dir: Direction) -> Option<Direction> {
    match dir {
        D => Some(LD),
        LD => Some(RD),
        RD => None,
    }
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            D => DOWN,
            LD => LEFTDOWN,
            RD => RIGHTDOWN,
        }
    }
}

fn step(map: &Map, pos: Position, dir: Option<Direction>) -> Position {
    if let Some(dir) = dir {
        let next_pos = pos + dir.into();
        match map.get(&next_pos) {
            None => next_pos,
            Some('#') | Some('o') => step(map, pos, next_dir(dir)),
            _ => panic!("This is not possible!"),
        }
    } else {
        pos
    }
}

fn fall(map: &mut Map, pos: Position, last_rock: isize) -> bool {
    let mut current_pos = pos;
    let mut next_pos = step(map, current_pos, Some(D));
    while next_pos != current_pos {
        if next_pos.y >= last_rock + 1 {
            return false;
        }
        current_pos = next_pos;
        next_pos = step(map, current_pos, Some(D));
    }
    map.insert(current_pos, 'o');
    true
}

impl Puzzle for Day14 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/14.input");
        for line in INPUT.lines() {
            let positions = parse_path(line).unwrap().1;
            for i in 0..positions.len() - 1 {
                for p in get_positions(positions[i], positions[i + 1]) {
                    self.input.insert(p, '#');
                }
            }
        }
    }

    fn part1(&self) -> String {
        let last_rock = self.input.keys().max().unwrap().y;
        let mut map = self.input.clone();
        let mut count = 0;
        let starting_pos = Position { x: 500, y: 0 };
        while fall(&mut map, starting_pos, last_rock) {
            count += 1;
        }
        format!("{:?}", count)
    }

    fn part2(&self) -> String {
        let last_rock = self.input.keys().max().unwrap().y;
        let mut map = self.input.clone();
        for x in -last_rock - 2..=last_rock + 2 {
            map.insert(
                Position {
                    x: 500 + x,
                    y: last_rock + 2,
                },
                '#',
            );
        }
        let mut count = 0;
        let starting_pos = Position { x: 500, y: 0 };
        while map.get(&starting_pos).is_none() && fall(&mut map, starting_pos, last_rock + 3) {
            count += 1;
        }
        //print_map(&map);
        format!("{:?}", count)
    }
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    let last_rock = map.keys().max().unwrap().y;
    let map_set = map.keys().collect::<HashSet<_>>();
    for y in 0..=last_rock {
        for x in 500 - last_rock - 1..=500 + last_rock + 1 {
            if map_set.contains(&Position { x, y }) {
                print!("{:}", map[&Position { x, y }]);
            } else {
                print!("{:}", '.');
            }
        }
        println!();
    }
    println!("---------------------------------------------------")
}
