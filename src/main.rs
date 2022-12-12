
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use aoc_2022_rust as lib;
use lib::Puzzle;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;

enum Selector {
    All,
    Single(usize)
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
    
    let mut days: Vec<&mut dyn Puzzle> = vec![&mut day1, &mut day2, &mut day3, &mut day4, &mut day5,
					      &mut day6, &mut day7, &mut day8, &mut day9, &mut day10,
					      &mut day11];
    
    match selection {
	Selector::Single(n) => lib::print_day(2022, n, days[n-1].run()),
	Selector::All => for n in 0..days.len() {
	    lib::print_day(2022, n+1, days[n].run());	    
	}
    }    
}
