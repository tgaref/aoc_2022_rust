use aoc_2022_rust::{Grid, Puzzle};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone)]
pub struct Day12 {
    input: Map,
    start: Position,
    goal: Position,
}

type Map = Grid<char>;
type Position = (isize, isize);

impl Day12 {
    pub fn new() -> Day12 {
        Day12 {
            input: Grid::new(),
            start: (0, 0),
            goal: (0, 0),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Grid::new();
        self.start = (0, 0);
        self.goal = (0, 0);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    pos: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on dist.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbours_up(grid: &Map, pos: Position) -> Vec<Position> {
    let mut neigh = Vec::new();
    let m = grid.dims.0 as isize;
    let n = grid.dims.1 as isize;
    let c = grid[pos.0 as usize][pos.1 as usize] as u8;
    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let p = (pos.0 + dir.0, pos.1 + dir.1);
        if p.0 >= 0
            && p.0 < m
            && p.1 >= 0
            && p.1 < n
            && grid[p.0 as usize][p.1 as usize] as u8 <= c + 1
        {
            neigh.push(p);
        }
    }
    neigh
}

fn dijkstra(grid: &Map, start: Position, goal: Position) -> Option<u64> {
    let (m, n) = grid.dims;
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<Position, u64> = HashMap::with_capacity(m * n);
    for i in 0isize..(m as isize) {
        for j in 0isize..(n as isize) {
            dist.insert((i, j), u64::MAX);
        }
    }

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        pos: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, pos }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if pos == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[&pos] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for node in neighbours_up(grid, pos) {
            let next = State {
                cost: cost + 1,
                pos: node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[&next.pos] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.pos, next.cost);
            }
        }
    }
    // Goal not reachable
    None
}

fn neighbours_down(grid: &Map, pos: Position) -> Vec<Position> {
    let mut neigh = Vec::new();
    let m = grid.dims.0 as isize;
    let n = grid.dims.1 as isize;
    let c = grid[pos.0 as usize][pos.1 as usize] as u8;
    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let p = (pos.0 + dir.0, pos.1 + dir.1);
        if p.0 >= 0
            && p.0 < m
            && p.1 >= 0
            && p.1 < n
            && grid[p.0 as usize][p.1 as usize] as u8 >= c - 1
        {
            neigh.push(p);
        }
    }
    neigh
}

fn all_shortest_paths(grid: &Map, start: Position) -> HashMap<Position, u64> {
    let (m, n) = grid.dims;
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<Position, u64> = HashMap::with_capacity(m * n);
    for i in 0isize..(m as isize) {
        for j in 0isize..(n as isize) {
            dist.insert((i, j), u64::MAX);
        }
    }

    let mut dd: HashMap<Position, u64> = HashMap::with_capacity(m * n);

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        pos: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, pos }) = heap.pop() {
        // If reached node is of interest, insert it in the HashMap
        if grid[pos.0 as usize][pos.1 as usize] == 'a' {
            dd.insert(pos, cost);
        }

        // Important as we may have already found a better way
        if cost > dist[&pos] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for node in neighbours_down(grid, pos) {
            let next = State {
                cost: cost + 1,
                pos: node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[&next.pos] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.pos, next.cost);
            }
        }
    }
    // Return HashMap of interesting distances
    dd
}

impl Puzzle for Day12 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/12.input");
        let mut rows = Vec::new();
        for (i, line) in INPUT.lines().enumerate() {
            let mut row = line.chars().collect::<Vec<char>>();
            for j in 0..row.len() {
                if row[j] == 'S' {
                    row[j] = 'a';
                    self.start = (i as isize, j as isize)
                } else if row[j] == 'E' {
                    row[j] = 'z';
                    self.goal = (i as isize, j as isize)
                }
            }
            rows.push(row);
        }
        self.input = Grid::from_rows(rows);
    }

    fn part1(&self) -> String {
        let d = dijkstra(&self.input, self.start, self.goal);
        format!("{:?}", d)
    }

    fn part2(&self) -> String {
        let dist = all_shortest_paths(&self.input, self.goal);
        format!("{:?}", dist.values().min())
    }
}
