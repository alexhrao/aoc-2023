use std::{fs, str::FromStr};

use regex::Regex;

use super::Day;

pub struct Day24;

const MIN: f64 = 200000000000000f64;
const MAX: f64 = 400000000000000f64;

#[derive(Debug, Clone, Copy, PartialEq)]
struct HailStone {
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
    x >= MIN && x <= MAX && y >= MIN && y <= MAX
}

impl Day for Day24 {
    fn task1(&self, file: &std::path::Path) {
        let stones: Vec<HailStone> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .inspect(|s: &HailStone| println!("y = {}x + {}", s.m(), s.b()))
            .collect();
        let total = stones
            .iter()
            .enumerate()
            .flat_map(|(s1, stone1)| {
                stones
                    .iter()
                    .skip(s1+1)
                    .filter_map(|stone2| {
                        let x = stone1.intersect(stone2);
                        if stone1.in_future(&x) && stone2.in_future(&x) {
                            Some(x)
                        } else {
                            None
                        }
                    })
            })
            .filter(in_range)
            .count();
        println!("{}", total);
    }
    fn task2(&self, _file: &std::path::Path) {}
}
