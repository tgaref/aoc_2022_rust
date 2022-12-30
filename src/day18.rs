use aoc_2022_rust::Puzzle;
use nom::{bytes::complete::tag, character::complete, combinator::map, sequence::tuple, IResult};
use std::collections::HashSet;

type Node = (i32, i32, i32);

#[derive(Debug, Clone)]
pub struct Day18 {
    input: HashSet<Node>,
}

impl Day18 {
    pub fn new() -> Day18 {
        Day18 {
            input: HashSet::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = HashSet::new()
    }
}

fn parse_line(input: &str) -> IResult<&str, Node> {
    map(
        tuple((
            complete::i32,
            tag(","),
            complete::i32,
            tag(","),
            complete::i32,
        )),
        |(a, _, b, _, c)| (a, b, c),
    )(input)
}

impl Puzzle for Day18 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/18.input");
        for line in INPUT.lines() {
            self.input.insert(parse_line(line).unwrap().1);
        }
    }

    fn part1(&self) -> String {
        format!("{:?}", faces(&self.input, &HashSet::new()))
    }

    fn part2(&self) -> String {
        let mut empty_space = HashSet::new();
        for x in -1..=20 {
            for y in -1..=20 {
                for z in -1..=20 {
                    if !self.input.contains(&(x, y, z)) {
                        empty_space.insert((x, y, z));
                    }
                }
            }
        }
        let interior = empty_space
            .difference(&connected_component(&self.input, (-1, -1, -1)))
            .copied()
            .collect::<HashSet<_>>();

        format!("{:?}", faces(&self.input, &interior))
    }
}

fn faces(input: &HashSet<Node>, exclude: &HashSet<Node>) -> usize {
    let mut count = 0;
    for (x, y, z) in input {
        for dir in &[
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let (x1, y1, z1) = (x + dir.0, y + dir.1, z + dir.2);
            if -1 <= x1
                && x1 <= 20
                && -1 <= y1
                && y1 <= 20
                && -1 <= z1
                && z1 <= 20
                && !input.contains(&(x1, y1, z1))
                && !exclude.contains(&(x1, y1, z1))
            {
                count += 1;
            }
        }
    }
    count
}

fn neighbours(input: &HashSet<Node>, (x, y, z): Node) -> Vec<Node> {
    let mut nei = Vec::new();
    for dir in &[
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ] {
        let (x1, y1, z1) = (x + dir.0, y + dir.1, z + dir.2);
        if -1 <= x1
            && x1 <= 20
            && -1 <= y1
            && y1 <= 20
            && -1 <= z1
            && z1 <= 20
            && !input.contains(&(x1, y1, z1))
        {
            nei.push((x1, y1, z1));
        }
    }
    nei
}

#[allow(dead_code)]
fn all_connected_components(
    input: &HashSet<Node>,
    empty_space: &HashSet<Node>,
) -> Vec<HashSet<Node>> {
    let mut done = HashSet::new();
    let mut components = Vec::new();

    for node in empty_space {
        if !done.contains(node) {
            let new_component = connected_component(input, *node);
            done = done.union(&new_component).copied().collect();
            components.push(new_component);
        }
    }
    components
}

fn connected_component(input: &HashSet<Node>, start: Node) -> HashSet<Node> {
    let mut component = HashSet::new();
    let mut queue = Vec::new();

    // Insert starting node in the queue
    queue.push(start);

    // While there are unexplored nodes...
    while let Some(node) = queue.pop() {
        component.insert(node);

        for pos in neighbours(input, node) {
            if !component.contains(&pos) {
                queue.push(pos)
            }
        }
    }
    component
}
