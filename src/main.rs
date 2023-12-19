use clap::Parser;
use std::path::{Path, PathBuf};

pub mod day01;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

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
        01 => Box::new(day01::Day01 {}),
        02 => Box::new(day02::Day02 {}),
        03 => Box::new(day03::Day03 {}),
        04 => Box::new(day04::Day04 {}),
        05 => Box::new(day05::Day05 {}),
        06 => Box::new(day06::Day06 {}),
        07 => Box::new(day07::Day07 {}),
        08 => Box::new(day08::Day08 {}),
        09 => Box::new(day09::Day09 {}),
        10 => Box::new(day10::Day10 {}),
        11 => Box::new(day11::Day11 {}),
        12 => Box::new(day12::Day12 {}),
        13 => Box::new(day13::Day13 {}),
        14 => Box::new(day14::Day14 {}),
        15 => Box::new(day15::Day15 {}),
        16 => Box::new(day16::Day16 {}),
        17 => Box::new(day17::Day17 {}),
        18 => Box::new(day18::Day18 {}),
        d => panic!("Unrecognized day {}", d),
    };
    if let Some(t1) = args.task1_file {
        day.task1(&t1);
    }
    if let Some(t2) = args.task2_file {
        day.task2(&t2);
    }
}
