use aoc_2022_rust::{Grid, Puzzle};

use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Day8 {
    input: Grid<isize>,
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 { input: Grid::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Grid::new()
    }
}

impl Puzzle for Day8 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/8.input");
        let mut grid = vec![];
        for line in INPUT.lines() {
            let row = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect::<Vec<_>>();
            grid.push(row);
        }
        self.input = Grid::from_rows(grid);
    }

    fn part1(&self) -> String {
        format!("{:?}", visible(&self.input))
    }

    fn part2(&self) -> String {
        format!("{:}", score(&self.input))
    }
}

fn visible(grid: &Grid<isize>) -> usize {
    let mut prev = -1;
    let mut set = HashSet::new();

    for i in 0..grid.dims.0 {
        let mut left = grid.row(1).enumerate();
        let mut right = grid.row(1).enumerate().rev();
        let iters: Vec<&mut dyn Iterator<Item = (usize, &isize)>> = vec![&mut left, &mut right];
        for iter in iters {
            for (j, v) in iter {
                if *v > prev {
                    set.insert((i, j));
                    prev = *v;
                }
            }
            prev = -1;
        }
    }

    for j in 0..grid.dims.1 {
        let mut up = grid.col(1).enumerate();
        let mut down = grid.col(1).enumerate().rev();
        let iters: Vec<&mut dyn Iterator<Item = (usize, &isize)>> = vec![&mut up, &mut down];
        for iter in iters {
            for (i, v) in iter {
                if *v > prev {
                    set.insert((i, j));
                    prev = *v;
                }
            }
            prev = -1;
        }
    }
    set.len()
}

const LEFT: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (0, 1);
const UP: (isize, isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);

fn in_bounds(pos: (isize, isize), dims: (usize, usize)) -> bool {
    pos.0 >= 0 && pos.0 < (dims.0 as isize) && pos.1 >= 0 && pos.1 < (dims.1 as isize)
}

fn score(grid: &Grid<isize>) -> isize {
    let mut win = 0;
    let (m, n) = grid.dims;
    for x in 1..m - 1 {
        for y in 1..n - 1 {
            let tree = grid[x][y];
            let mut current_tree_score = 1;
            for dir in [LEFT, RIGHT, UP, DOWN] {
                let mut pos: (isize, isize) = (x as isize, y as isize);
                let mut vis = 0;
                pos = (pos.0 + dir.0, pos.1 + dir.1);
                while in_bounds(pos, (m, n)) && grid[pos.0 as usize][pos.1 as usize] < tree {
                    vis += 1;
                    pos = (pos.0 + dir.0, pos.1 + dir.1);
                }
                if in_bounds(pos, (m, n)) {
                    vis += 1
                }
                current_tree_score *= vis;
            }
            win = win.max(current_tree_score);
        }
    }
    win
}
