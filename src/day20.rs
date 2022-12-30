use aoc_2022_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day20 {
    input: Vec<Element>,
}

impl Day20 {
    pub fn new() -> Day20 {
        Day20 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Element {
    val: i64,
    serial: usize,
}

impl Puzzle for Day20 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/20.input");
        for (i, line) in INPUT.lines().enumerate() {
            self.input.push(Element {
                val: line.parse::<i64>().unwrap(),
                serial: i,
            });
        }
    }

    fn part1(&self) -> String {
        let mut list = self.input.clone();
        mix(&mut list);
        let i = list
            .iter()
            .enumerate()
            .find(|(_, Element { val, serial: _ })| *val == 0)
            .unwrap()
            .0;

        format!(
            "{:?}",
            list[(i + 1000) % list.len()].val
                + list[(i + 2000) % list.len()].val
                + list[(i + 3000) % list.len()].val
        )
    }

    fn part2(&self) -> String {
        let key = 811589153;
        let mut list = self
            .input
            .clone()
            .iter()
            .map(|Element { val, serial }| Element {
                val: val * key,
                serial: *serial,
            })
            .collect::<Vec<_>>();

        for _ in 0..10 {
            mix(&mut list);
        }

        let i = list
            .iter()
            .enumerate()
            .find(|(_, Element { val, serial: _ })| *val == 0)
            .unwrap()
            .0;

        format!(
            "{:?}",
            list[(i + 1000) % list.len()].val
                + list[(i + 2000) % list.len()].val
                + list[(i + 3000) % list.len()].val
        )
    }
}

fn cyclic_shift_left<T: Copy>(v: &mut [T]) {
    let n = v.len();
    let tmp = v[0];
    for i in 0..n - 1 {
        v[i] = v[i + 1];
    }
    v[n - 1] = tmp;
}

fn cyclic_shift_right<T: Copy>(v: &mut [T]) {
    let n = v.len();
    let tmp = v[n - 1];
    for i in 1..n {
        v[n - i] = v[n - i - 1];
    }
    v[0] = tmp;
}

fn compute_slice(i: usize, v: &mut [Element]) {
    let n = v.len();
    let val = v[i].val % (n as i64 - 1);

    if val < 0 {
        if i as i64 + val - 1 >= 0 {
            let j = (i as i64 + val) as usize;
            cyclic_shift_right(&mut v[j..i + 1]);
        } else {
            let j = val.rem_euclid(n as i64) as usize;
            cyclic_shift_left(&mut v[i..i + j]);
        }
    } else if val > 0 {
        if i + (val as usize) < n {
            let j = val as usize + 1;
            cyclic_shift_left(&mut v[i..i + j]);
        } else {
            let j = (i as i64 + val - n as i64 + 1) as usize;
            cyclic_shift_right(&mut v[j..i + 1]);
        }
    }
}

fn mix(v: &mut [Element]) {
    let mut current = 0;
    for _ in 0..v.len() {
        let i = v
            .iter()
            .enumerate()
            .find(|(_, Element { val: _, serial })| *serial == current)
            .unwrap()
            .0;
        compute_slice(i, v);
        current += 1;
    }
}
