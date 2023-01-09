use aoc_2022_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day25 {
    input: Vec<Vec<i64>>,
}

impl Day25 {
    pub fn new() -> Day25 {
        Day25 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new();
    }
}

fn char_to_digit(c: char) -> i64 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        _ => panic!("Unknown sysmbol!"),
    }
}

fn digit_to_char(d: i64) -> char {
    match d {
        0 => '0',
        1 => '1',
        2 => '2',
        -1 => '-',
        -2 => '=',
        _ => panic!("Unknown sysmbol!"),
    }
}

impl Puzzle for Day25 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/25.input");
        for line in INPUT.lines() {
            self.input
                .push(line.chars().map(|c| char_to_digit(c)).collect::<Vec<_>>());
        }
    }

    fn part1(&self) -> String {
        let suma: i64 = self.input.iter().map(|snafu| snafu_to_int(snafu)).sum();

        let code = int_to_snafu(suma)
            .iter()
            .map(|d| digit_to_char(*d))
            .collect::<String>();

        format!("{:?}", code)
    }

    fn part2(&self) -> String {
        format!("{:?}", "Not needed!")
    }
}

fn snafu_to_int(snafu: &[i64]) -> i64 {
    snafu.iter().fold(0, |acc, d| acc * 5 + d)
}

fn int_to_snafu(mut m: i64) -> Vec<i64> {
    let mut snafu = vec![];
    while m != 0 {
        let d = match m % 5 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => -2,
            4 => -1,
            _ => panic!("Impossible!"),
        };
        snafu.push(d);
        m = (m - d).div_euclid(5);
    }
    snafu.into_iter().rev().collect::<Vec<_>>()
}
