use std::{collections::HashMap, fs};

use super::Day;
use gcd::{self, Gcd};
use regex::Regex;

pub struct Day08;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

fn traverse_single(dirs: &[Direction], map: &HashMap<&str, (&str, &str)>) -> usize {
    let mut seed = "AAA";
    let mut counter: usize = 0;
    for dir in dirs.iter().cycle() {
        let choices = map[seed];
        seed = match dir {
            Direction::Left => choices.0,
            Direction::Right => choices.1,
        };
        counter += 1;
        if seed == "ZZZ" {
            break;
        }
    }
    counter
}

// Naive...
fn traverse(start: &str, dirs: &[Direction], map: &HashMap<&str, (&str, &str)>) -> usize {
    let mut seed = start;
    let mut counter: usize = 0;
    for dir in dirs.iter().cycle() {
        let choices = map[seed];
        seed = match dir {
            Direction::Left => choices.0,
            Direction::Right => choices.1,
        };
        counter += 1;
        if seed.ends_with('Z') {
            break;
        }
    }
    counter
}

fn traverse_ghost(dirs: &[Direction], map: &HashMap<&str, (&str, &str)>) -> usize {
    let mut times = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| traverse(k, dirs, map));

    let mut ans = times.next().unwrap();
    for t in times {
        ans = t * ans / t.gcd(ans);
    }
    ans
}

impl Day for Day08 {
    fn task1(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();

        let mut lines = backing.lines();
        let dirs: Vec<Direction> = lines.next().unwrap().chars().map(char::into).collect();
        // println!("{:?}", dirs);
        let reg = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").unwrap();
        let nodes: HashMap<_, _> = lines
            .skip(1)
            .map(|l| {
                let m = reg.captures(l).unwrap();
                (
                    m.get(1).unwrap().as_str(),
                    (m.get(2).unwrap().as_str(), m.get(3).unwrap().as_str()),
                )
            })
            .collect();
        // println!("{:?}", nodes);
        println!("{}", traverse_single(&dirs, &nodes));
    }
    fn task2(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();

        let mut lines = backing.lines();
        let dirs: Vec<Direction> = lines.next().unwrap().chars().map(char::into).collect();
        let reg = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").unwrap();
        let nodes: HashMap<_, _> = lines
            .skip(1)
            .map(|l| {
                let m = reg.captures(l).unwrap();
                (
                    m.get(1).unwrap().as_str(),
                    (m.get(2).unwrap().as_str(), m.get(3).unwrap().as_str()),
                )
            })
            .collect();
        println!("{}", traverse_ghost(&dirs, &nodes));
    }
}
