mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use aoc_2022_rust as lib;
use lib::Puzzle;

use day1::Day1;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day2::Day2;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;

enum Selector {
    All,
    Single(usize),
}

fn main() {
    let mut args = std::env::args().skip(1);

    let selection = if args.len() == 0 {
        Selector::All
    } else {
        Selector::Single(args.next().unwrap().parse::<usize>().unwrap())
    };

    let mut day1 = Day1::new();
    let mut day2 = Day2::new();
    let mut day3 = Day3::new();
    let mut day4 = Day4::new();
    let mut day5 = Day5::new();
    let mut day6 = Day6::new();
    let mut day7 = Day7::new();
    let mut day8 = Day8::new();
    let mut day9 = Day9::new();
    let mut day10 = Day10::new();
    let mut day11 = Day11::new();
    let mut day12 = Day12::new();
    let mut day13 = Day13::new();
    let mut day14 = Day14::new();
    let mut day15 = Day15::new();
    let mut day16 = Day16::new();
    let mut day17 = Day17::new();
    let mut day18 = Day18::new();
    let mut day19 = Day19::new();
    let mut day20 = Day20::new();
    let mut day21 = Day21::new();
    let mut day22 = Day22::new();
    let mut day23 = Day23::new();
    let mut day24 = Day24::new();
    let mut day25 = Day25::new();

    let mut days: Vec<&mut dyn Puzzle> = vec![
        &mut day1, &mut day2, &mut day3, &mut day4, &mut day5, &mut day6, &mut day7, &mut day8,
        &mut day9, &mut day10, &mut day11, &mut day12, &mut day13, &mut day14, &mut day15,
        &mut day16, &mut day17, &mut day18, &mut day19, &mut day20, &mut day21, &mut day22,
        &mut day23, &mut day24, &mut day25,
    ];

    match selection {
        Selector::Single(n) => lib::print_day(2022, n, days[n - 1].run()),
        Selector::All => {
            for n in 0..days.len() {
                lib::print_day(2022, n + 1, days[n].run());
            }
        }
    }
}
