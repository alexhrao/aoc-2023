use clap::Parser;
use std::path::PathBuf;

pub mod day1;
pub mod day2;

pub trait Day {
    fn task1(&self, file: &PathBuf);
    fn task2(&self, file: &PathBuf);
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
        d => panic!("Unrecognized day {}", d),
    };
    if let Some(t1) = args.task1_file {
        day.task1(&t1);
    }
    if let Some(t2) = args.task2_file {
        day.task2(&t2);
    }
}
