use aoc_2022_rust::Puzzle;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Day23 {
    input: Map,
}

type Direction = (isize, isize);

type Position = (isize, isize);

type Map = HashSet<Position>;

impl Day23 {
    pub fn new() -> Day23 {
        Day23 {
            input: HashSet::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = HashSet::new();
    }
}

impl Puzzle for Day23 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/23.input");
        for (i, line) in INPUT.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    self.input.insert((i as isize, j as isize));
                }
            }
        }
    }

    fn part1(&self) -> String {
        let mut map = self.input.clone();
        let mut idx = 0;

        for _ in 0..10 {
            (map, _) = round(&map, idx);
            idx = (idx + 1) % 4;
        }

        let max_i = map.iter().max_by_key(|(i, _)| i).unwrap().0;
        let min_i = map.iter().min_by_key(|(i, _)| i).unwrap().0;
        let max_j = map.iter().max_by_key(|(_, j)| j).unwrap().1;
        let min_j = map.iter().min_by_key(|(_, j)| j).unwrap().1;

        let empty_cells = (max_i - min_i + 1) * (max_j - min_j + 1) - map.len() as isize;

        //print_map(&map);
        format!("{:?}", empty_cells)
    }

    fn part2(&self) -> String {
        let mut map = self.input.clone();
        let mut new_map;
        let mut idx = 0;
        let mut k = 0u64;
        let mut moved: bool;
        loop {
            (new_map, moved) = round(&map, idx);
            k += 1;
            if !moved {
                break;
            }
            map = new_map;
            idx = (idx + 1) % 4;
        }

        format!("{:?}", k)
    }
}

const N: Direction = (-1, 0);
const S: Direction = (1, 0);
const W: Direction = (0, -1);
const E: Direction = (0, 1);
const NW: Direction = (-1, -1);
const NE: Direction = (-1, 1);
const SW: Direction = (1, -1);
const SE: Direction = (1, 1);

const DIR: &[Direction] = &[N, S, W, E];

fn round(map: &Map, idx: usize) -> (Map, bool) {
    let mut proposed_map: HashMap<Position, Vec<Position>> = HashMap::new();
    for pos in map {
        let new_pos = proposed_position(&map, *pos, idx);
        if let Some(x) = proposed_map.get_mut(&new_pos) {
            x.push(*pos);
        } else {
            proposed_map.insert(new_pos, vec![*pos]);
        }
    }

    let mut new_map = HashSet::new();
    let mut moved = false;
    for (new_pos, prev_positions) in proposed_map {
        if prev_positions.len() == 1 {
            new_map.insert(new_pos);
            if new_pos != prev_positions[0] {
                moved = true;
            }
        } else {
            for pos in prev_positions {
                new_map.insert(pos);
            }
        }
    }
    (new_map, moved)
}

fn check_dir(map: &Map, (i, j): Position, dir: Direction) -> bool {
    let new_pos = (i + dir.0, j + dir.1);
    let positions = if dir.1 == 0 {
        [
            new_pos,
            (new_pos.0, new_pos.1 - 1),
            (new_pos.0, new_pos.1 + 1),
        ]
    } else {
        [
            new_pos,
            (new_pos.0 - 1, new_pos.1),
            (new_pos.0 + 1, new_pos.1),
        ]
    };
    positions.iter().all(|pos| !map.contains(pos))
}

fn proposed_position(map: &Map, (i, j): Position, idx: usize) -> Position {
    let empty_spaces = [N, S, W, E, NW, NE, SW, SE]
        .iter()
        .all(|(x, y)| !map.contains(&(i + x, j + y)));
    if empty_spaces {
        return (i, j);
    }
    for k in 0..4 {
        let dir = DIR[(idx + k) % 4];
        if check_dir(map, (i, j), dir) {
            return (i + dir.0, j + dir.1);
        }
    }
    (i, j)
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    let max_i = map.iter().max_by_key(|(i, _)| i).unwrap().0;
    let min_i = map.iter().min_by_key(|(i, _)| i).unwrap().0;
    let max_j = map.iter().max_by_key(|(_, j)| j).unwrap().1;
    let min_j = map.iter().min_by_key(|(_, j)| j).unwrap().1;

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if map.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
