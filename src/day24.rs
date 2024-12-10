use aoc_runner_derive::{aoc, aoc_generator};
use std::{fmt::Display, str::FromStr};

use regex::Regex;

const MIN: f64 = 200_000_000_000_000_f64;
const MAX: f64 = 400_000_000_000_000_f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HailStone {
    posn: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl Display for HailStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z) = self.posn;
        let (vx, vy, vz) = self.velocity;
        write!(f, "{x}, {y}, {z} @ {vx}, {vy}, {vz}")
    }
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
    fn x(&self) -> f64 {
        self.posn.0
    }
    fn y(&self) -> f64 {
        self.posn.1
    }
    fn z(&self) -> f64 {
        self.posn.2
    }
    fn vx(&self) -> f64 {
        self.velocity.0
    }
    fn vy(&self) -> f64 {
        self.velocity.1
    }
    fn vz(&self) -> f64 {
        self.velocity.2
    }
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

fn rref<const M: usize, const N: usize>(mut matrix: [[f64; N]; M]) -> [f64; M] {
    let mut h = 0;
    let mut k = 0;
    while h < M && k < N {
        // Find kth pivot
        let mut max = (h, matrix[h][k].abs());
        for (i, row) in matrix[h..].iter().enumerate() {
            let tmp = row[k].abs();
            if tmp > max.1 {
                max = (i, tmp);
            }
        }
        let i_max = max.0;
        if matrix[i_max][k] == 0f64 {
            k += 1;
        } else {
            // Swap the rows
            matrix.swap(h, i_max);
            for i in h + 1..M {
                let f = matrix[i][k] / matrix[h][k];
                matrix[i][k] = 0f64;
                for j in k + 1..N {
                    matrix[i][j] -= f * matrix[h][j];
                }
            }
            h += 1;
            k += 1;
        }
    }
    // Reduce it
    for r in (0..M).rev() {
        for c in r + 1..M {
            matrix[r][N - 1] -= matrix[r][c] * matrix[c][N - 1];
            matrix[r][c] = 0f64;
        }
        matrix[r][N - 1] = (matrix[r][N - 1] / matrix[r][r]).round();
        matrix[r][r] = 1f64;
    }

    let mut out = [0f64; M];
    for r in 0..M {
        out[r] = matrix[r][N - 1];
    }
    out
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

#[aoc(day24, part2)]
pub fn part2(stones: &[HailStone]) -> f64 {
    // This code is **heavily** based on code from ash42:
    //
    // https://github.com/ash42/adventofcode/blob/main/adventofcode2023/src/nl/michielgraat/adventofcode2023/day24/Day24.java
    let mut matrix = [[0f64; 5]; 4];
    for (r, ss) in stones[..5].windows(2).enumerate() {
        let [s1, s2] = ss else { unreachable!() };
        matrix[r][0] = s2.vy() - s1.vy();
        matrix[r][1] = s1.vx() - s2.vx();
        matrix[r][2] = s1.y() - s2.y();
        matrix[r][3] = s2.x() - s1.x();
        matrix[r][4] = -s1.x() * s1.vy() + s1.y() * s1.vx() + s2.x() * s2.vy() - s2.y() * s2.vx();
    }
    let [x, y, vx, vy] = rref(matrix);
    let mut matrix = [[0f64; 3]; 2];
    for (r, ss) in stones[..3].windows(2).enumerate() {
        let [s1, s2] = ss else { unreachable!() };
        matrix[r][0] = s1.vx() - s2.vx();
        matrix[r][1] = s2.x() - s1.x();
        matrix[r][2] = -s1.x() * s1.vz() + s1.z() * s1.vx() + s2.x() * s2.vz()
            - s2.z() * s2.vx()
            - ((s2.vz() - s1.vz()) * x)
            - ((s1.z() - s2.z()) * vx);
    }
    let [z, vz] = rref(matrix);
    let stone = HailStone {
        posn: (x, y, z),
        velocity: (vx, vy, vz),
    };
    println!("Thrown: {stone}");
    (x + y + z).round()
}
