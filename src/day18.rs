use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

use crate::day10::{Grid, Tile};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' | '3' => Direction::Up,
            'R' | '0' => Direction::Right,
            'D' | '1' => Direction::Down,
            'L' | '2' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            Ok(s.chars().next().unwrap().into())
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanRecord {
    dir: Direction,
    len: usize,
    color: String,
}

impl PlanRecord {
    fn explode(&self) -> impl Iterator<Item = Direction> + '_ {
        std::iter::repeat(self.dir).take(self.len)
    }
}

impl std::ops::Add<&PlanRecord> for (isize, isize) {
    type Output = (isize, isize);
    fn add(self, rhs: &PlanRecord) -> Self::Output {
        match rhs.dir {
            Direction::Down => (self.0 + rhs.len as isize, self.1),
            Direction::Left => (self.0, self.1 - rhs.len as isize),
            Direction::Right => (self.0, self.1 + rhs.len as isize),
            Direction::Up => (self.0 - rhs.len as isize, self.1),
        }
    }
}

impl FromStr for PlanRecord {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([RDLU]) (\d+) \(#([0-9a-f]{6})\)").unwrap();
        let caps = re.captures(s).unwrap();
        let dir = caps.get(1).unwrap().as_str().parse().unwrap();
        let len = caps.get(2).unwrap().as_str().parse().unwrap();
        let color = caps.get(3).unwrap().as_str().to_string();

        Ok(Self { dir, len, color })
    }
}

impl std::ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Down => (self.0 + 1, self.1),
            Direction::Left => (self.0, self.1 - 1),
            Direction::Right => (self.0, self.1 + 1),
            Direction::Up => (self.0 - 1, self.1),
        }
    }
}

impl std::ops::Add<Direction> for Direction {
    type Output = Tile;
    fn add(self, rhs: Direction) -> Self::Output {
        (self, rhs).into()
    }
}

impl From<(Direction, Direction)> for Tile {
    fn from(value: (Direction, Direction)) -> Self {
        match value {
            (Direction::Up | Direction::Down, Direction::Up | Direction::Down) => Tile::NS,
            (Direction::Left | Direction::Right, Direction::Left | Direction::Right) => Tile::EW,
            (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => Tile::SW,
            (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => Tile::SE,
            (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => Tile::NW,
            (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => Tile::NE,
        }
    }
}

fn dig(plan: &[PlanRecord]) -> (Grid, usize) {
    let mut trench = vec![];
    let mut posn = (0isize, 0isize);
    for (curr, next) in plan.iter().flat_map(PlanRecord::explode).tuple_windows() {
        posn = posn + curr;
        trench.push((posn, curr + next));
    }
    let last = plan.last().unwrap().dir + plan.first().unwrap().dir;
    trench.push(((0, 0), last));
    let base_row = *trench.iter().map(|((r, _), _)| r).min().unwrap();
    let base_col = *trench.iter().map(|((_, c), _)| c).min().unwrap();
    let trench_len = trench.len();
    let augmented = trench
        .into_iter()
        .map(|((r, c), t)| (((r - base_row) as usize, (c - base_col) as usize), t));
    (Grid::from(augmented), trench_len)
}

fn vertices(plans: &[PlanRecord]) -> Vec<(isize, isize)> {
    // Start at 0, 0
    let mut point = (0isize, 0isize);
    let mut out = vec![];
    for plan in plans {
        point = point + plan;
        out.push(point);
    }
    out
}

#[aoc_generator(day18)]
pub fn gen(input: &str) -> Vec<PlanRecord> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day18, part1)]
pub fn part1(plans: &[PlanRecord]) -> usize {
    let (dig, trench) = dig(plans);
    dig.num_contained() + trench
}

#[aoc(day18, part2)]
pub fn part2(plans: &[PlanRecord]) -> isize {
    let verts = vertices(plans);

    let area = verts
        .windows(2)
        .map(|vv| (vv[0], vv[1]))
        // Connect back to where we started
        .chain(std::iter::once((
            *verts.last().unwrap(),
            *verts.first().unwrap(),
        )))
        .map(|(v1, v2)| (v1.1 - v2.1) * v1.0)
        .sum::<isize>();
    let perim = verts
        .windows(2)
        .map(|vv| (vv[0], vv[1]))
        // Connect back to where we started
        .chain(std::iter::once((
            *verts.last().unwrap(),
            *verts.first().unwrap(),
        )))
        .map(|(v1, v2)| (v1.1.abs_diff(v2.1) + v1.0.abs_diff(v2.0)) as isize)
        .sum::<isize>();
    // We've only added half the perimeter! And we haven't included the top left square
    area + perim / 2 + 1
}
