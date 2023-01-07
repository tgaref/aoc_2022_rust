use aoc_2022_rust::Puzzle;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Day24 {
    input: Ring,
    dims: (isize, isize),
}

type Blizzards = HashMap<Bliz, HashSet<Position>>;

type TakenPositions = HashSet<Position>;

type Position = (isize, isize);

type Ring = Vec<TakenPositions>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Bliz {
    R,
    L,
    U,
    D,
    Other,
}

use Bliz::{Other, D, L, R, U};

impl Bliz {
    fn from_char(c: char) -> Bliz {
        match c {
            '>' => R,
            '<' => L,
            '^' => U,
            'v' => D,
            _ => Other,
        }
    }
}

impl Day24 {
    pub fn new() -> Day24 {
        Day24 {
            input: Vec::new(),
            dims: (0, 0),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new();
        self.dims = (0, 0);
    }
}

impl Puzzle for Day24 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/24.input");
        let rows = (INPUT.lines().count() - 2) as isize;
        let cols = (INPUT.lines().take(1).next().unwrap().chars().count() - 2) as isize;
        let mut input = HashMap::new();
        input.insert(R, HashSet::new());
        input.insert(L, HashSet::new());
        input.insert(U, HashSet::new());
        input.insert(D, HashSet::new());

        for (i, line) in INPUT.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if let Some(x) = input.get_mut(&Bliz::from_char(c)) {
                    x.insert((i as isize, j as isize));
                }
            }
        }
        self.dims = (rows, cols);
        let prering = compute_ring(input, self.dims);
        self.input = prering
            .iter()
            .map(|blizzard| convert_blizzard_to_set(blizzard))
            .collect::<Vec<_>>();
    }

    fn part1(&self) -> String {
        let start = State::new(0, (0, 1), 0);
        let end = bfs(
            &self.input,
            start,
            self.dims,
            (self.dims.0 + 1, self.dims.1),
        );

        format!("{:?}", end.time)
    }

    fn part2(&self) -> String {
        let init = (0, 1);
        let finish = (self.dims.0 + 1, self.dims.1);
        let start = State::new(0, init, 0);
        let end = bfs(&self.input, start, self.dims, finish);
        let start = bfs(&self.input, end, self.dims, init);
        let end = bfs(&self.input, start, self.dims, finish);
        format!("{:?}", end.time)
    }
}

fn convert_blizzard_to_set(blizzard: &Blizzards) -> TakenPositions {
    let mut union: TakenPositions = HashSet::new();
    for dir in &[R, L, U, D] {
        union.extend(&blizzard[dir]);
    }
    union
}

fn compute_ring(blizzard: Blizzards, dims: (isize, isize)) -> Vec<Blizzards> {
    let mut ring = Vec::new();
    let mut newblizzard = next(&blizzard, dims);
    ring.push(blizzard.clone());
    while newblizzard != blizzard {
        ring.push(newblizzard.clone());
        newblizzard = next(&newblizzard, dims);
    }
    ring
}

fn next(blizzards: &Blizzards, dims: (isize, isize)) -> Blizzards {
    let mut newblizzards = HashMap::new();
    for (bliz, positions) in blizzards {
        newblizzards.insert(
            *bliz,
            positions
                .iter()
                .map(|pos| advance(bliz, dims, *pos))
                .collect::<HashSet<_>>(),
        );
    }
    newblizzards
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Map {
    idx: usize,
    me: Position,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    map: Map,
    time: usize,
}

impl State {
    fn new(idx: usize, me: Position, time: usize) -> State {
        State {
            map: Map { idx, me },
            time,
        }
    }
}

fn bfs(ring: &Ring, start: State, dims: (isize, isize), goal: Position) -> State {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back(start);
    seen.insert(start.map);
    // While there are unexplored states...
    while let Some(state) = queue.pop_front() {
        // If the goal is reached, return
        if state.map.me == goal {
            return state;
        } else {
            for new_state in next_states(ring, &state, dims) {
                if !seen.contains(&new_state.map) {
                    queue.push_back(new_state);
                    seen.insert(new_state.map);
                }
            }
        }
    }
    panic!("Failed...")
}

fn next_states(ring: &Ring, state: &State, dims: (isize, isize)) -> Vec<State> {
    let mut new_states = Vec::new();
    let new_idx = (state.map.idx + 1) % ring.len();
    for (x, y) in [(-1, 0), (0, -1), (0, 0), (1, 0), (0, 1)] {
        let new_me = (state.map.me.0 + x, state.map.me.1 + y);
        if in_bounds(dims, new_me) && !ring[new_idx].contains(&new_me) {
            new_states.push(State::new(new_idx, new_me, state.time + 1));
        }
    }
    new_states
}

fn in_bounds((m, n): (isize, isize), (i, j): Position) -> bool {
    (i >= 1 && i <= m && j >= 1 && j <= n) || (i == 0 && j == 1) || (i == m + 1 && j == n)
}

fn advance(bliz: &Bliz, (m, n): (isize, isize), (i, j): Position) -> Position {
    match bliz {
        R => {
            if j < n {
                (i, j + 1)
            } else {
                (i, 1)
            }
        }
        L => {
            if j > 1 {
                (i, j - 1)
            } else {
                (i, n)
            }
        }
        U => {
            if i > 1 {
                (i - 1, j)
            } else {
                (m, j)
            }
        }
        D => {
            if i < m {
                (i + 1, j)
            } else {
                (1, j)
            }
        }
        Other => panic!("Should not happen!"),
    }
}
