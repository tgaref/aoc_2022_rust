use aoc_2022_rust::Puzzle;
use nom::{
    branch::alt,
    character::complete::{self, char},
    combinator::map,
    multi::many1,
    sequence::pair,
    IResult,
};
use std::collections::HashMap;

type Map = Vec<Vec<char>>;

#[derive(Debug, Clone)]
pub struct Day22 {
    instructions: Vec<Instruction>,
    faces: HashMap<usize, Map>,
}

impl Day22 {
    pub fn new() -> Day22 {
        Day22 {
            instructions: Vec::new(),
            faces: HashMap::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.instructions = Vec::new();
        self.faces = HashMap::new();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    steps: u32,
    turn: char,
}

fn parse_direction(input: &str) -> IResult<&str, Instruction> {
    map(
        pair(alt((char('R'), char('L'))), complete::u32),
        |(turn, steps)| Instruction { steps, turn },
    )(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_direction)(input)
}

impl Puzzle for Day22 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/22.input");
        let n = 50;
        let mut face1 = Vec::new();
        let mut face2 = Vec::new();
        let mut face3 = Vec::new();
        let mut face4 = Vec::new();
        let mut face5 = Vec::new();
        let mut face6 = Vec::new();
        for (i, line) in INPUT.lines().enumerate() {
            let iter = line.chars();
            if i < 50 {
                let f = iter.clone().skip(n).take(n).collect::<Vec<_>>();
                face1.push(f);
                let f = iter.clone().skip(2 * n).take(n).collect::<Vec<_>>();
                face2.push(f);
            }
            if i >= 50 && i < 100 {
                let f = iter.clone().skip(n).take(n).collect::<Vec<_>>();
                face3.push(f)
            }
            if i >= 100 && i < 150 {
                let f = iter.clone().take(n).collect::<Vec<_>>();
                face5.push(f);
                let f = iter.clone().skip(n).take(n).collect::<Vec<_>>();
                face4.push(f);
            }
            if i >= 150 && i < 200 {
                let f = iter.clone().take(n).collect::<Vec<_>>();
                face6.push(f)
            }

            if line.is_empty() {
                continue;
            }

            if i == 201 {
                self.instructions = parse_directions(line).unwrap().1;
            }
        }
        self.faces.insert(1, face1);
        self.faces.insert(2, face2);
        self.faces.insert(3, face3);
        self.faces.insert(4, face4);
        self.faces.insert(5, face5);
        self.faces.insert(6, face6);
    }

    fn part1(&self) -> String {
        let mut state = State {
            face: 1,
            pos: (0, 0),
            dir: Up,
        };

        for instruction in &self.instructions {
            state = exec(1, 50, &self.faces, state, *instruction)
        }
        let (i, j) = compute_coodrinates(state.face, state.pos);
        let passwd = 1000 * (i + 1) + 4 * (j + 1) + value(state.dir);

        format!("{:?}", passwd)
    }

    fn part2(&self) -> String {
        let mut state = State {
            face: 1,
            pos: (0, 0),
            dir: Up,
        };

        for instruction in &self.instructions {
            state = exec(2, 50, &self.faces, state, *instruction)
        }
        let (i, j) = compute_coodrinates(state.face, state.pos);
        let passwd = 1000 * (i + 1) + 4 * (j + 1) + value(state.dir);
        format!("{:?}", passwd)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::{Down, Left, Right, Up};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    face: usize,
    pos: (isize, isize),
    dir: Direction,
}

impl State {
    fn new(face: usize, pos: (isize, isize), dir: Direction) -> State {
        State { face, pos, dir }
    }
}

fn compute_coodrinates(face: usize, (i, j): (isize, isize)) -> (isize, isize) {
    match face {
        1 => (i, j + 50),
        2 => (i, j + 100),
        3 => (i + 50, j + 50),
        4 => (i + 100, j + 50),
        5 => (i + 100, j),
        6 => (i + 150, j),
        _ => panic!("Unrecognized face!"),
    }
}

fn value(dir: Direction) -> isize {
    match dir {
        Up => 3,
        Down => 1,
        Right => 0,
        Left => 2,
    }
}

fn exec(
    part: usize,
    n: isize,
    faces: &HashMap<usize, Map>,
    mut state: State,
    instruction: Instruction,
) -> State {
    let dir = match instruction.turn {
        'R' => turn_right(state.dir),
        'L' => turn_left(state.dir),
        _ => panic!("Unrecognized turn!"),
    };

    state.dir = dir;
    for _ in 0..instruction.steps {
        let new_state = next(part, n, faces, state);
        if new_state.pos == state.pos {
            return state;
        }
        state = new_state
    }

    state
}

fn edge_check(n: isize, state: State) -> bool {
    match state.dir {
        Up => state.pos.0 == 0,
        Down => state.pos.0 == n - 1,
        Right => state.pos.1 == n - 1,
        Left => state.pos.1 == 0,
    }
}

fn next(part: usize, n: isize, faces: &HashMap<usize, Map>, state: State) -> State {
    let teleport = if part == 1 { teleport1 } else { teleport2 };
    let State {
        face,
        pos: (i, j),
        dir,
    } = state;

    let (di, dj) = match dir {
        Up => (-1, 0),
        Down => (1, 0),
        Right => (0, 1),
        Left => (0, -1),
    };

    if edge_check(n, state) {
        let State {
            face: f,
            pos: (new_i, new_j),
            dir: new_dir,
        } = teleport(n, state);
        if faces[&f][new_i as usize][new_j as usize] == '#' {
            return State::new(f, (i, j), dir);
        } else {
            return State::new(f, (new_i, new_j), new_dir);
        }
    } else if faces[&face][(i + di) as usize][(j + dj) as usize] == '#' {
        return state;
    } else {
        return State::new(face, (i + di, j + dj), dir);
    }
}

fn teleport1(n: isize, state: State) -> State {
    let State {
        face,
        pos: (i, j),
        dir,
    } = state;
    match face {
        1 => match dir {
            Up => State::new(4, (n - 1, j), Up),
            Down => State::new(3, (0, j), Down),
            Right => State::new(2, (i, 0), Right),
            Left => State::new(2, (i, n - 1), Left),
        },
        2 => match dir {
            Up => State::new(2, (n - 1, j), Up),
            Down => State::new(2, (0, j), Down),
            Right => State::new(1, (i, 0), Right),
            Left => State::new(1, (i, n - 1), Left),
        },
        3 => match dir {
            Up => State::new(1, (n - 1, j), Up),
            Down => State::new(4, (0, j), Down),
            Right => State::new(3, (i, 0), Right),
            Left => State::new(3, (i, n - 1), Left),
        },
        4 => match dir {
            Up => State::new(3, (n - 1, j), Up),
            Down => State::new(1, (0, j), Down),
            Right => State::new(5, (i, 0), Right),
            Left => State::new(5, (i, n - 1), Left),
        },
        5 => match dir {
            Up => State::new(6, (n - 1, j), Up),
            Down => State::new(6, (0, j), Down),
            Right => State::new(4, (i, 0), Right),
            Left => State::new(4, (i, n - 1), Left),
        },
        6 => match dir {
            Up => State::new(5, (n - 1, j), Up),
            Down => State::new(5, (0, j), Down),
            Right => State::new(6, (i, 0), Right),
            Left => State::new(6, (i, n - 1), Left),
        },
        _ => panic!("The face is not recognized!"),
    }
}

fn teleport2(n: isize, state: State) -> State {
    let State {
        face,
        pos: (i, j),
        dir,
    } = state;

    match face {
        1 => match dir {
            Up => State::new(6, (j, 0), Right),
            Down => State::new(3, (0, j), Down),
            Right => State::new(2, (i, 0), Right),
            Left => State::new(5, (n - 1 - i, 0), Right),
        },
        2 => match dir {
            Up => State::new(6, (n - 1, j), Up),
            Down => State::new(3, (j, n - 1), Left),
            Right => State::new(4, (n - 1 - i, n - 1), Left),
            Left => State::new(1, (i, n - 1), Left),
        },
        3 => match dir {
            Up => State::new(1, (n - 1, j), Up),
            Down => State::new(4, (0, j), Down),
            Right => State::new(2, (n - 1, i), Up),
            Left => State::new(5, (0, i), Down),
        },
        4 => match dir {
            Up => State::new(3, (n - 1, j), Up),
            Down => State::new(6, (j, n - 1), Left),
            Right => State::new(2, (n - 1 - i, n - 1), Left),
            Left => State::new(5, (i, n - 1), Left),
        },
        5 => match dir {
            Up => State::new(3, (j, 0), Right),
            Down => State::new(6, (0, j), Down),
            Right => State::new(4, (i, 0), Right),
            Left => State::new(1, (n - 1 - i, 0), Right),
        },
        6 => match dir {
            Up => State::new(5, (n - 1, j), Up),
            Down => State::new(2, (0, j), Down),
            Right => State::new(4, (n - 1, i), Up),
            Left => State::new(1, (0, i), Down),
        },
        _ => panic!("The face is not recognized!"),
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Up => Left,
        Right => Up,
        Down => Right,
        Left => Down,
    }
}
