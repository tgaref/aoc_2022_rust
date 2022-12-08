use aoc_2022_rust::{Puzzle, Grid};

use std::collections::{HashSet};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Day8 {
    input: Grid<isize>
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
	    let row = line.chars().map(|c| c.to_digit(10).unwrap() as isize).collect::<Vec<_>>();
	    grid.push(row);
	}
	let dims = (grid.len(), grid[0].len());
	self.input = Grid::from_rows(grid, dims);
    }

    fn part1(&self)-> String {
	format!("{:?}", visible(&self.input))
    }      				    

    fn part2(&self) -> String {
        format!("{:}", score(&self.input))
    }        
}

fn visible(grid: &Grid<isize>) -> usize
{
    let mut prev = -1;
    let mut set = HashSet::new();
    for i in 0..grid.dims.0 {
	for (j,v) in grid.row(i).enumerate() {
	    if *v > prev {
		set.insert((i,j));
		prev = *v;		
	    } 
	}
	prev = -1;
	for (j,v) in grid.row(i).enumerate().rev() {
	    if *v > prev {
		set.insert((i,j));
		prev = *v;		
	    } 
	}
	prev = -1;
    }
    for j in 0..grid.dims.1 {
	for (i,v) in grid.col(j).enumerate() {
	    if *v > prev {
		set.insert((i,j));
		prev = *v;		
	    } 
	}
	prev = -1;
	for (i,v) in grid.col(j).enumerate().rev() {
	    if *v > prev {
		set.insert((i,j));
		prev = *v;		
	    } 
	}
	prev = -1;
    }
    set.len()
}

fn score(grid: &Grid<isize>) -> isize {
    let mut win = 0;
    let (m,n) = grid.dims;
    for x in 1..m-1 {
	for y in 1..n-1 {
	    let tree = grid[x][y];
	    let mut left = 0;
	    let mut j = (y-1) as isize;
	    while j >= 0 && grid[x][j as usize] < tree {
		left += 1;
		j -= 1;
	    }
	    if j >= 0 {
		left += 1
	    }

	    let mut j = y+1;
	    let mut right = 0;
	    while j < n && grid[x][j] < tree {
		right += 1;
		j += 1;
	    }
	    if j < n {
		right += 1
	    }

	    let mut up = 0;
	    let mut i = (x-1) as isize;
	    while i >= 0 && grid[i as usize][y] < tree {
		up += 1;
		i -= 1;
	    }
	    if i >= 0 {
		up += 1
	    }

	    let mut i = x+1;
	    let mut down = 0;
	    while i < n && grid[i][y] < tree {
		down += 1;
		i += 1;
	    }
	    if i < n {
		down += 1
	    }

	    win = win.max(left * right * up * down);
	}
    }
    win
}


