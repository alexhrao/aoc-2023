use aoc_runner_derive::aoc;
use std::{collections::HashSet, fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Garden {
    dims: (usize, usize),
    rocks: HashSet<(usize, usize)>,
    spaces: HashSet<(usize, usize)>,
    visited: HashSet<(usize, usize)>,
    current: HashSet<(usize, usize)>,
}

impl Garden {
    pub fn step(&mut self) {
        let mut current = HashSet::new();
        for posn in self.current.drain() {
            // try going left, right, up, down
            let (r, c) = posn;
            if r > 0 {
                // Try up
                let new = (r - 1, c);
                if !self.rocks.contains(&new) {
                    self.visited.insert(new);
                    current.insert(new);
                }
            }
            if r < self.dims.0 - 1 {
                // Try down
                let new = (r + 1, c);
                if !self.rocks.contains(&new) {
                    self.visited.insert(new);
                    current.insert(new);
                }
            }
            if c > 0 {
                // Try left
                let new = (r, c - 1);
                if !self.rocks.contains(&new) {
                    self.visited.insert(new);
                    current.insert(new);
                }
            }
            if c < self.dims.1 - 1 {
                let new = (r, c + 1);
                if !self.rocks.contains(&new) {
                    self.visited.insert(new);
                    current.insert(new);
                }
            }
        }
        self.current = current;
    }
}

impl Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.dims.0 {
            for c in 0..self.dims.1 {
                if self.current.contains(&(r, c)) {
                    write!(f, "O")?;
                } else if self.spaces.contains(&(r, c)) {
                    write!(f, ".")?;
                } else if self.rocks.contains(&(r, c)) {
                    write!(f, "#")?;
                } else {
                    write!(f, "S")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Garden {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spaces = s
            .lines()
            .enumerate()
            .flat_map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(c, ch)| if ch == '.' { Some((r, c)) } else { None })
            })
            .collect();
        let rocks = s
            .lines()
            .enumerate()
            .flat_map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(c, ch)| if ch == '#' { Some((r, c)) } else { None })
            })
            .collect();
        let start = s
            .lines()
            .enumerate()
            .find_map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .find_map(|(c, ch)| if ch == 'S' { Some((r, c)) } else { None })
            })
            .unwrap();

        let dims = (s.lines().count(), s.lines().next().unwrap().chars().count());
        Ok(Garden {
            dims,
            spaces,
            rocks,
            visited: HashSet::from([start]),
            current: HashSet::from([start]),
        })
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    let mut garden: Garden = input.parse().unwrap();
    for _ in 0..26_501_365 {
        garden.step();
    }
    garden.current.len()
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    let mut garden: Garden = input.parse().unwrap();
    for _ in 0..26_501_365 {
        garden.step();
    }
    garden.current.len()
}
