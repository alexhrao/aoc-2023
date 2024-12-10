use aoc_runner_derive::aoc;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    ops::Rem,
    str::FromStr,
};

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
        let mut current = HashSet::with_capacity(4 * self.current.len());
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
    for _ in 0..64 {
        garden.step();
    }
    garden.current.len()
}

pub struct Garden2 {
    rows: usize,
    cols: usize,
    rocks: HashSet<(usize, usize)>,
    start: (usize, usize),
}

impl FromStr for Garden2 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

        Ok(Self {
            rows: s.lines().count(),
            cols: s.lines().next().unwrap().len(),
            rocks,
            start,
        })
    }
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    // Note: A year later I finally gave up and looked to the internet for hints.
    //  It turns out that, as somewhat expected, this had very little to do with
    //  coding. It turns out that the geometry of the input -- a diamond with
    //  "radius" of ~65 -- is important. This solution is not my own, but is
    //  heavily adapted from:
    //
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    //
    //  Which I highly recommend reading!
    let garden: Garden2 = input.parse().unwrap();

    // BFS
    let visited = {
        let mut visited = HashMap::new();
        // Queue of nodes to explore; start with our starting address
        let mut to_explore = VecDeque::new();
        to_explore.push_back((0usize, garden.start));

        // Main thrust of BFS
        while let Some((dist, posn)) = to_explore.pop_front() {
            // If we've already been here, no need to keep looking
            if visited.contains_key(&posn) {
                continue;
            }
            // We haven't been here, so by definition, we couldn't have gotten here
            // sooner
            visited.insert(posn, dist);

            // Look left, right, up, down, and add positions to our queue to explore
            // try going left, right, up, down
            let (r, c) = posn;
            if r > 0 {
                // Try up
                let new = (r - 1, c);
                // If I haven't been here, and it's not a rock, go for it!
                if !garden.rocks.contains(&new) && !visited.contains_key(&new) {
                    to_explore.push_back((dist + 1, new));
                }
            }
            if r < garden.rows - 1 {
                // Try down
                let new = (r + 1, c);
                // If I haven't been here, and it's not a rock, go for it!
                if !garden.rocks.contains(&new) && !visited.contains_key(&new) {
                    to_explore.push_back((dist + 1, new));
                }
            }
            if c > 0 {
                // Try left
                let new = (r, c - 1);
                // If I haven't been here, and it's not a rock, go for it!
                if !garden.rocks.contains(&new) && !visited.contains_key(&new) {
                    to_explore.push_back((dist + 1, new));
                }
            }
            if c < garden.cols - 1 {
                let new = (r, c + 1);
                // If I haven't been here, and it's not a rock, go for it!
                if !garden.rocks.contains(&new) && !visited.contains_key(&new) {
                    to_explore.push_back((dist + 1, new));
                }
            }
        }
        // Visited now holds our BFS result
        visited
    };

    // I honestly don't really understand the math (and I don't think the author
    //  does either). Basically you have to cut out some corners that just happen
    //  to fall on odd squares, and include some corners that are on even squares
    //  (each square is a tiled version of the original garden)
    // 65 == radius of our map (131 total width/height)
    let radius = garden.cols / 2;
    assert_eq!(radius, 65);
    let evens = visited
        .values()
        .filter(|d| **d > radius && d.rem(2) == 0)
        .count();
    let odds = visited
        .values()
        .filter(|d| **d > radius && d.rem(2) == 1)
        .count();

    // The above link has a really great description of how they got to this number.
    //  In a nutshell, subtract off the "radius" and then divide by the width
    let n = (26501365 - radius) / garden.cols;
    assert_eq!(n, 202300);

    // # Odd * odd parity tiles + # Even * even parity tiles. Subtract off the
    //  odd corners and add back the even ones... I guess... this part I still
    //  don't truly understand.
    let odd = (n + 1) * (n + 1) * visited.values().filter(|d| d.rem(2) == 1).count();
    let even = n * n * visited.values().filter(|d| d.rem(2) == 0).count();
    odd + even - ((n + 1) * odds) + (n * evens)
}
