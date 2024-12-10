use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

use regex::Regex;

const MIN: f64 = 200_000_000_000_000_f64;
const MAX: f64 = 400_000_000_000_000_f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HailStone {
    posn: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl FromStr for HailStone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"(-?\d+),\s+(-?\d+),\s+(-?\d+)\s+@\s+([-]?\d+),\s+([-]?\d+),\s+([-]?\d+)")
                .unwrap();
        if let Some(caps) = re.captures(s) {
            let nums: Vec<_> = (1..=6)
                .map(|g| caps.get(g).unwrap().as_str().parse().unwrap())
                .collect();
            Ok(HailStone {
                posn: (nums[0], nums[1], nums[2]),
                velocity: (nums[3], nums[4], nums[5]),
            })
        } else {
            Err(())
        }
    }
}

impl HailStone {
    fn m(&self) -> f64 {
        self.velocity.1 / self.velocity.0
    }
    fn b(&self) -> f64 {
        self.posn.1 - (self.m() * self.posn.0)
    }
    pub fn intersect(&self, other: &HailStone) -> (f64, f64) {
        // x coord
        let m = self.m() - other.m();
        let b = other.b() - self.b();
        let x = b / m;
        let y = self.m() * x + self.b();
        (x, y)
    }
    pub fn in_future(&self, pt: &(f64, f64)) -> bool {
        // Linear, so. If the sign of x dir same as sign of pt.x - self.x, great!
        self.velocity.0.is_sign_positive() == (pt.0 - self.posn.0).is_sign_positive()
    }
}

fn in_range(xy: &(f64, f64)) -> bool {
    let &(x, y) = xy;
    let rng = MIN..=MAX;
    rng.contains(&x) && rng.contains(&y)
}

#[aoc_generator(day24)]
pub fn gen(input: &str) -> Vec<HailStone> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day24, part1)]
pub fn part1(stones: &[HailStone]) -> usize {
    stones
        .iter()
        .enumerate()
        .flat_map(|(s1, stone1)| {
            stones.iter().skip(s1 + 1).filter_map(|stone2| {
                let x = stone1.intersect(stone2);
                if stone1.in_future(&x) && stone2.in_future(&x) {
                    Some(x)
                } else {
                    None
                }
            })
        })
        .filter(in_range)
        .count()
}
