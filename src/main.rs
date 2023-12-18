use clap::Parser;
use std::path::{Path, PathBuf};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub trait Day {
    fn task1(&self, file: &Path);
    fn task2(&self, file: &Path);
}

#[derive(Parser, Debug)]
struct Args {
    day: u8,
    #[arg(long = "t1")]
    task1_file: Option<PathBuf>,
    #[arg(long = "t2")]
    task2_file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let day: Box<dyn Day> = match args.day {
        1 => Box::new(day1::Day1 {}),
        2 => Box::new(day2::Day2 {}),
        3 => Box::new(day3::Day3 {}),
        4 => Box::new(day4::Day4 {}),
        5 => Box::new(day5::Day5 {}),
        6 => Box::new(day6::Day6 {}),
        7 => Box::new(day7::Day7 {}),
        8 => Box::new(day8::Day8 {}),
        9 => Box::new(day9::Day9 {}),
        10 => Box::new(day10::Day10 {}),
        11 => Box::new(day11::Day11 {}),
        12 => Box::new(day12::Day12 {}),
        13 => Box::new(day13::Day13 {}),
        14 => Box::new(day14::Day14 {}),
        15 => Box::new(day15::Day15 {}),
        16 => Box::new(day16::Day16 {}),
        d => panic!("Unrecognized day {}", d),
    };
    if let Some(t1) = args.task1_file {
        day.task1(&t1);
    }
    if let Some(t2) = args.task2_file {
        day.task2(&t2);
    }
}
