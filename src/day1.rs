use super::Day;
use std::fs;

pub struct Day1 {}

const NUMS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn word_to_num(word: &str) -> u32 {
    let idx = NUMS.iter().position(|&w| w == word).unwrap() as u32;
    if idx > 8 {
        idx - 8
    } else {
        idx + 1
    }
}

fn get_digits(line: &str) -> u32 {
    let mut first = 0;
    for c in line.chars() {
        if c.is_ascii_digit() {
            first = c.to_digit(10).unwrap();
            break;
        }
    }
    let mut second = 0;
    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            second = c.to_digit(10).unwrap();
            break;
        }
    }
    (first * 10) + second
}

fn get_digits_hard(line: &str) -> u32 {
    let first = word_to_num(
        NUMS.iter()
            .filter_map(|&w| line.find(w).map(|i| (i, w)))
            .min_by_key(|&(i, _)| i)
            .unwrap()
            .1,
    );
    let second = word_to_num(
        NUMS.iter()
            .filter_map(|&w| line.rfind(w).map(|i| (i, w)))
            .max_by_key(|&(i, _)| i)
            .unwrap()
            .1,
    );
    (first * 10) + second
}

impl Day for Day1 {
    fn task1(&self, file: &std::path::Path) {
        let sum: u32 = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(get_digits)
            .sum();
        println!("{}", sum);
    }
    fn task2(&self, file: &std::path::Path) {
        let sum: u32 = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(get_digits_hard)
            .sum();
        println!("{}", sum);
    }
}
