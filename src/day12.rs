use std::{str::FromStr, fs, fmt::Display};
use indicatif::ParallelProgressIterator;

use rayon::iter::*;
use itertools::*;

use super::Day;

pub struct Day12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            _ => unreachable!()
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Operational => '.',
            Self::Damaged => '#',
            Self::Unknown => '?',
        })
    }
}

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Record {
    fn has_unknown(&self) -> bool {
        self.springs.iter().any(|s| s == &Spring::Unknown)
    }
    // pub fn is_valid(&self) -> bool {
    //     let springs: Vec<_> = self.springs
    //         .split(|s| s == &Spring::Operational)
    //         .filter(|s| s.len() > 0)
    //         .collect();
    //     if springs.len() != self.groups.len() {
    //         return false;
    //     }
    //     for (s, g) in springs.into_iter().zip(&self.groups) {
    //         if *g != s.len() {
    //             return false;
    //         }
    //     }
    //     true
    // }
    pub fn not_invalid(&self) -> bool {
        // Like is_valid, but now we have unknowns. Basically,
        // with what we currently know, can we already throw
        // out this one?
        // Iterate from group to group. We know we start from the
        // left. As soon as we see unknowns, bail
        let total_broken = self.groups.iter().sum::<usize>();
        // If number of broken > total broken, die
        let num_broken = self.springs.iter().filter(|&s| s == &Spring::Damaged).count();
        if num_broken > total_broken {
            return false;
        }
        let num_unknown = self.springs.iter().filter(|&s| s == &Spring::Unknown).count();
        // If the number of unknowns + number of damaged < total_broken, we're dead
        if (num_broken + num_unknown) < total_broken {
            return false;
        }
        let springs: Vec<_> = self.springs
            .split(|s| s == &Spring::Operational)
            .filter(|s| s.len() > 0)
            .collect();
        // If number of KNOWN groups is > self.groups.len, we're done
        if springs.iter().filter(|s| !s.contains(&Spring::Unknown)).count() > self.groups.len() {
            return false;
        }
        for (s, g) in springs.into_iter().zip(&self.groups) {
            // Regardless of unknowns, do we already have too many springs in this group?
            if s.iter().take_while(|&s| s != &Spring::Unknown).count() > *g {
                return false;
            }
            if s.iter().any(|s| s == &Spring::Unknown) {
                // Bail, we no longer know
                break
            }
            // This group is complete; it had better match the group size!
            if s.len() != *g {
                return false;
            }
        }
        true
    }
    pub fn naive_possibilities(&self) -> Vec<Record> {
        // Naive: Just all possibilities
        // Better: Before moving on, am I already invalid? If so, empty!
        if !self.not_invalid() {
            return vec![]
        }
        // Base case: Nobody is unknown. Just return ourself
        let mut clone = self.clone();
        if !self.has_unknown() {
            vec![clone]
        } else {
            // 2 possibilties: Broken, Fixed.
            // Set our bit and then move on
            let s = clone.springs.iter()
                .position(|s| s == &Spring::Unknown)
                .unwrap();
            clone.springs[s] = Spring::Damaged;
            let out = clone.naive_possibilities();
            clone.springs[s] = Spring::Operational;
            out
                .into_iter()
                .chain(clone.naive_possibilities())
                .collect()
        }
    }
    pub fn additive_possibilities(&self) -> usize {
        if !self.not_invalid() {
            return 0
        }
        if !self.has_unknown() {
            1
        } else {
            // println!("{}", self);
            let mut clone = self.clone();
            play(&mut clone);
            let s = clone.springs.iter()
                .position(|s| s == &Spring::Unknown)
                .unwrap();
            clone.springs[s] = Spring::Damaged;
            let out = clone.additive_possibilities();
            clone.springs[s] = Spring::Operational;
            out + clone.additive_possibilities()
        }
    }
}

impl FromStr for Record {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let springs = split.next().unwrap()
            .chars()
            .map(|c| c.into())
            .collect();
        let groups = split.next().unwrap()
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

// Naive: Generate all possibilities, then pare down to what is actually
// possible

impl Day for Day12 {
    fn task1(&self, file: &std::path::Path) {
        let records: Vec<Record> = fs::read_to_string(file).unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        println!("{}", records.iter().map(|r| r.naive_possibilities().len()).sum::<usize>());
    }
    fn task2(&self, file: &std::path::Path) {
        let mut records: Vec<Record> = fs::read_to_string(file).unwrap()
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
        // play(&mut records[0]);
        let stuff = records.iter()
            .map(|r| r.additive_possibilities())
            .sum::<usize>();
        println!("{}", stuff);
    }
}

fn split_springs(springs: &[Spring]) -> impl Iterator<Item = &[Spring]> {
    springs
        .split(|s| s == &Spring::Operational || s == &Spring::Unknown)
        .filter(|s| s.len() > 0)

}

fn play(record: &mut Record) {
    let max_group = *record.groups.iter().max().unwrap();
    // ## Pretend only one max group
    // If I already contain the max group, I know two things:
    // 1. Any question marks AROUND that group MUST be operational;
    // 2. There can be no other groups of that size
    let num_max_damaged = split_springs(&record.springs)
        .filter_map(|ss| if ss.iter().all(|s| s == &Spring::Damaged) { Some(ss.len()) } else { None })
        .filter(|sz| sz == &max_group)
        .count();
    let num_max_groups = record.groups.iter().filter(|&&g| g == max_group).count();
    if num_max_damaged == num_max_groups {
        // Look at each group and fill in around it
        let mut num_seen = 0usize;
        let mut start = 0usize;
        // println!("Before: {}", record);
        for s in 0..record.springs.len() {
            if record.springs[s] == Spring::Damaged {
                num_seen += 1;
            } else {
                start = s;
                num_seen = 0;
            }

            if num_seen == max_group {
                // fill in once before me, and once after me
                if let Some(e) = record.springs.get_mut(start) {
                    *e = Spring::Operational;
                }
                if let Some(e) = record.springs.get_mut(s + 1) {
                    *e = Spring::Operational;
                }
            }
        }
    }
    // println!("After: {}", record);
}