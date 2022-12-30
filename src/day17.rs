use aoc_2022_rust::Puzzle;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Day17 {
    input: String,
}

impl Day17 {
    pub fn new() -> Day17 {
        Day17 {
            input: String::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = String::new()
    }
}

type Map = HashSet<(usize, usize)>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    block: Block,
    grid: Map,
    highest: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block(Vec<(usize, usize)>);

impl Block {
    fn new(p: Vec<(usize, usize)>) -> Block {
        Block(p)
    }
}

fn move_down(state: State) -> (State, bool) {
    for (x, y) in &state.block.0 {
        if *x == 1 || state.grid.contains(&(x - 1, *y)) {
            return (state, false);
        }
    }
    let new_block = state
        .block
        .0
        .iter()
        .map(|(x, y)| (x - 1, *y))
        .collect::<Vec<_>>();

    (
        State {
            block: Block(new_block),
            grid: state.grid,
            highest: state.highest,
        },
        true,
    )
}

fn move_sideways(state: State, dir: char) -> State {
    match dir {
        '>' => {
            for (x, y) in &state.block.0 {
                if *y == 6 || state.grid.contains(&(*x, y + 1)) {
                    return state;
                }
            }
            let new_block = state
                .block
                .0
                .iter()
                .map(|(x, y)| (*x, y + 1))
                .collect::<Vec<_>>();
            State {
                block: Block(new_block),
                ..state
            }
        }
        '<' => {
            for (x, y) in &state.block.0 {
                if *y == 0 || state.grid.contains(&(*x, y - 1)) {
                    return state;
                }
            }
            let new_block = state
                .block
                .0
                .iter()
                .map(|(x, y)| (*x, y - 1))
                .collect::<Vec<_>>();
            State {
                block: Block(new_block),
                ..state
            }
        }
        _ => panic!("Not valid direction!"),
    }
}

fn initialize_block(block: &Block, highest: usize) -> Block {
    Block(
        block
            .0
            .iter()
            .map(|(x, y)| (x + highest + 4, y + 2))
            .collect(),
    )
}

fn simulate(input: &str, blocks: Vec<Block>, nrocks: usize) -> usize {
    let mut block = initialize_block(&blocks[0], 0);
    let mut state = State {
        block,
        grid: HashSet::new(),
        highest: 0,
    };
    let mut moved: bool;
    let mut blk_idx = 1;
    for dir in input.chars().cycle() {
        state = move_sideways(state, dir);
        (state, moved) = move_down(state);
        if !moved {
            let blk_highest = state.block.0.iter().max_by_key(|(x, _)| *x).unwrap().0;
            let highest = state.highest.max(blk_highest);
            let mut grid = state.grid;
            for (x, y) in state.block.0 {
                grid.insert((x, y));
            }
            block = initialize_block(&blocks[blk_idx % 5], highest);
            blk_idx += 1;
            state = State {
                block,
                grid,
                highest,
            };

            if blk_idx == nrocks + 1 {
                return state.highest;
            }
        }
    }
    state.highest
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Repeat {
    tops: Vec<usize>,
    input_idx: usize,
}

fn detect_cycle(input: &[char], blocks: Vec<Block>, nrocks: usize) -> usize {
    let mut block = initialize_block(&blocks[0], 0);
    let mut state = State {
        block,
        grid: HashSet::new(),
        highest: 0,
    };
    let mut moved: bool;
    let mut blk_idx = 0;
    let mut input_idx = 0;
    let mut highest;
    let mut computed_hight = 0;

    let mut observed = HashMap::new();
    let repeat = Repeat {
        tops: vec![0, 0, 0, 0, 0, 0, 0],
        input_idx: 0,
    };
    observed.insert(repeat, (0, 0));
    let mut tops = vec![0, 0, 0, 0, 0, 0, 0];
    let mut found = false;
    loop {
        let dir = input[input_idx];
        state = move_sideways(state, dir);
        (state, moved) = move_down(state);

        if !moved {
            // If enough blocks have fallen, return the highest point
            if blk_idx == nrocks {
                return computed_hight + state.highest;
            }

            // Compute the repeat-state
            let mut new_tops = tops.clone();
            for i in 0..7 {
                let block_max_x = state
                    .block
                    .0
                    .iter()
                    .filter(|(_, y)| *y == i)
                    .max_by_key(|(x, _)| *x)
                    .unwrap_or(&(0, 0))
                    .0;
                new_tops[i] = new_tops[i].max(block_max_x);
            }

            let base = tops.iter().min().unwrap();
            new_tops = new_tops.into_iter().map(|x| x - base).collect();

            let highest_x = state.block.0.iter().max_by_key(|(x, _)| *x).unwrap().0;
            highest = state.highest.max(highest_x);

            // The block came to rest. It is made part of the grid.
            let mut grid = state.grid;
            for (x, y) in state.block.0 {
                grid.insert((x, y));
            }

            let repeat = Repeat {
                tops: new_tops.clone(),
                input_idx,
            };

            // Setup tops for next iteration
            tops = new_tops;

            // Have we seen this repeat-state before?
            if observed.get(&repeat).is_some() && found == false {
                // If so, set found variable to true
                found = true;
                // Compute the hight of repeats of the cylce
                let rock_diff = blk_idx - observed[&repeat].0;
                let hight_diff = highest - observed[&repeat].1;
                let remain = nrocks - blk_idx;
                computed_hight = (remain / rock_diff) * hight_diff;
                // Set the new blk_idx to the point at the end of the cycles
                blk_idx = nrocks - (remain % rock_diff);
            } else {
                // If not, insert new repeat-state to oserved states
                observed.insert(repeat, (blk_idx, highest));
            }
            // Advance the blk_idx
            blk_idx += 1;

            // Prepare the new block
            block = initialize_block(&blocks[blk_idx % 5], highest);

            // Prepare the new state
            state = State {
                block,
                grid,
                highest,
            };
        }
        input_idx = (input_idx + 1) % input.len();
    }
    //    state.highest
}

impl Puzzle for Day17 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/17.input");
        self.input = INPUT.to_string();
    }

    fn part1(&self) -> String {
        let blocks: Vec<Block> = vec![
            Block::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            Block::new(vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
            Block::new(vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
            Block::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
            Block::new(vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        ];

        let h = simulate(&self.input, blocks, 2022);
        //	print_grid(&h.grid);
        format!("{:?}", h)
    }

    fn part2(&self) -> String {
        let blocks: Vec<Block> = vec![
            Block::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            Block::new(vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
            Block::new(vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
            Block::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
            Block::new(vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        ];

        let h = detect_cycle(
            &self.input.chars().collect::<Vec<_>>(),
            blocks,
            1000000000000,
        );
        format!("{:?}", h)
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Map) {
    for x in 0..=10 {
        let x = 10 - x;
        print!("{:2} |", x);
        for y in 0..7 {
            let c = if grid.contains(&(x, y)) { '#' } else { '.' };
            print!("{:}", c);
        }
        println!();
    }
}
