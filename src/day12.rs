use std::{collections::HashMap, fmt::Display, fs, str::FromStr};

use super::Day;

use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str,
};

pub struct Day12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '?' => Spring::Unknown,
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            _ => unreachable!(),
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Operational => '.',
                Self::Damaged => '#',
                Self::Unknown => '?',
            }
        )
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Record {
    pub fn possibilities(&self) -> usize {
        if self.springs.is_empty() {
            usize::from(self.groups.is_empty())
        } else if self.groups.is_empty() {
            usize::from(!self.springs.contains(&Spring::Damaged))
        } else {
            // At this point, we need to head to the base case.
            let mut result = 0;
            let head = self.springs.first().unwrap();
            if matches!(head, Spring::Unknown | Spring::Operational) {
                // If I'm operational, (or one side of the don't know), then no group is affected.
                // So in this case it's just the possibilities of everything to the right
                let mut clone = self.clone();
                clone.springs = clone.springs.into_iter().skip(1).collect();
                result += clone.possibilities();
            }

            if matches!(head, Spring::Unknown | Spring::Damaged) {
                let g = self.groups[0];
                let n_springs = self
                    .springs
                    .iter()
                    .take(g)
                    .all(|s| s != &Spring::Operational);
                let last_oper = self.springs.get(g);
                let last_oper = if let Some(o) = last_oper {
                    o != &Spring::Damaged
                } else {
                    true
                };
                if g <= self.springs.len() && n_springs && last_oper {
                    // We are allowed to consume the next group
                    let mut clone = self.clone();
                    // Add one to consume the separator
                    clone.springs = clone
                        .springs
                        .into_iter()
                        .skip(clone.groups[0] + 1)
                        .collect();
                    clone.groups = clone.groups.into_iter().skip(1).collect();
                    result += clone.possibilities();
                }
            }
            result
        }
    }

    pub fn memoized_possibilities(&self, cache: &mut HashMap<Record, usize>) -> usize {
        if let Some(ans) = cache.get(self) {
            return *ans;
        }
        if self.springs.is_empty() {
            usize::from(self.groups.is_empty())
        } else if self.groups.is_empty() {
            usize::from(!self.springs.contains(&Spring::Damaged))
        } else {
            // At this point, we need to head to the base case.
            let mut result = 0;
            let head = self.springs.first().unwrap();
            if matches!(head, Spring::Unknown | Spring::Operational) {
                // If I'm operational, (or one side of the don't know), then no group is affected.
                // So in this case it's just the possibilities of everything to the right
                let mut clone = self.clone();
                clone.springs = clone.springs.into_iter().skip(1).collect();
                result += clone.memoized_possibilities(cache);
            }

            if matches!(head, Spring::Unknown | Spring::Damaged) {
                let g = self.groups[0];
                let n_springs = self
                    .springs
                    .iter()
                    .take(g)
                    .all(|s| s != &Spring::Operational);
                let last_oper = self.springs.get(g);
                let last_oper = if let Some(o) = last_oper {
                    o != &Spring::Damaged
                } else {
                    true
                };
                if g <= self.springs.len() && n_springs && last_oper {
                    // We are allowed to consume the next group
                    let mut clone = self.clone();
                    // Add one to consume the separator
                    clone.springs = clone
                        .springs
                        .into_iter()
                        .skip(clone.groups[0] + 1)
                        .collect();
                    clone.groups = clone.groups.into_iter().skip(1).collect();
                    result += clone.memoized_possibilities(cache);
                }
            }
            cache.insert(self.clone(), result);
            result
        }
    }
}

impl FromStr for Record {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let springs = split
            .next()
            .unwrap()
            .chars()
            .map(std::convert::Into::into)
            .collect();
        let groups = split
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Record { springs, groups })
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let springs: Vec<String> = self.springs.iter().map(Spring::to_string).collect();
        let groups: Vec<String> = self.groups.iter().map(usize::to_string).collect();
        write!(f, "{} {}", springs.join(""), groups.join(","))
    }
}

impl Day for Day12 {
    fn task1(&self, file: &std::path::Path) {
        let records: Vec<Record> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let result = records.iter().map(Record::possibilities).sum::<usize>();
        println!("{result}");
    }
    fn task2(&self, file: &std::path::Path) {
        let records: Vec<Record> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .map(|mut r: Record| {
                let springs = r.springs.clone();
                for _ in 0..4 {
                    r.springs.push(Spring::Unknown);
                    r.springs.extend(&springs);
                }
                r.groups = r.groups.repeat(5);
                r
            })
            .collect();
        let result = records
            .par_iter()
            .map(|r| r.memoized_possibilities(&mut HashMap::new()))
            .sum::<usize>();
        println!("{result}");
    }
}
