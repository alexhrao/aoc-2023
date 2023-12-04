use super::Day;
use std::{fs, ops::AddAssign, str::FromStr};

pub struct Day2;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cube {
    Blue(usize),
    Red(usize),
    Green(usize),
}
impl FromStr for Cube {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: usize = s.split_whitespace().next().unwrap().parse().unwrap();
        if s.ends_with("blue") {
            Ok(Cube::Blue(num))
        } else if s.ends_with("red") {
            Ok(Cube::Red(num))
        } else if s.ends_with("green") {
            Ok(Cube::Green(num))
        } else {
            panic!("Unknown cube {}", s)
        }
    }
}

impl std::ops::Add for Cube {
    type Output = Cube;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Cube::Blue(c1), Cube::Blue(c2)) => Cube::Blue(c1 + c2),
            (Cube::Red(c1), Cube::Red(c2)) => Cube::Red(c1 + c2),
            (Cube::Green(c1), Cube::Green(c2)) => Cube::Green(c1 + c2),
            _ => panic!("Cannot add two different cube types!"),
        }
    }
}

#[derive(Debug, Clone)]
struct SetSummary {
    red: Cube,
    green: Cube,
    blue: Cube,
}

impl SetSummary {
    pub fn new() -> Self {
        SetSummary {
            red: Cube::Red(0),
            green: Cube::Green(0),
            blue: Cube::Blue(0),
        }
    }

    pub fn mult(&self) -> usize {
        let r = match self.red {
            Cube::Red(r) => r,
            _ => panic!(),
        };
        let g = match self.green {
            Cube::Green(g) => g,
            _ => panic!(),
        };
        let b = match self.blue {
            Cube::Blue(b) => b,
            _ => panic!(),
        };
        r * g * b
    }
}

impl std::iter::Sum for SetSummary {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut seed = SetSummary::new();
        for c in iter {
            seed += c;
        }
        seed
    }
}

impl std::ops::Add for SetSummary {
    type Output = SetSummary;
    fn add(self, rhs: Self) -> Self::Output {
        SetSummary {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign for SetSummary {
    fn add_assign(&mut self, rhs: Self) {
        self.red = self.red + rhs.red;
        self.green = self.green + rhs.green;
        self.blue = self.blue + rhs.blue;
    }
}

impl From<Cube> for SetSummary {
    fn from(value: Cube) -> Self {
        match value {
            Cube::Red(r) => Self {
                red: Cube::Red(r),
                green: Cube::Green(0),
                blue: Cube::Blue(0),
            },
            Cube::Green(g) => Self {
                red: Cube::Red(0),
                green: Cube::Green(g),
                blue: Cube::Blue(0),
            },
            Cube::Blue(b) => Self {
                red: Cube::Red(0),
                green: Cube::Green(0),
                blue: Cube::Blue(b),
            },
        }
    }
}

impl From<&Set> for SetSummary {
    fn from(value: &Set) -> Self {
        value.cubes.iter().map(|&c| c.into()).sum()
    }
}

#[derive(Debug, Clone)]
struct Set {
    cubes: Vec<Cube>,
}

impl FromStr for Set {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s.split(",").map(|c| c.trim().parse().unwrap()).collect();
        Ok(Self { cubes })
    }
}
#[derive(Debug, Clone)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.strip_prefix("Game ").unwrap().split(":");
        let id = split.next().unwrap().parse().unwrap();
        let sets = split
            .next()
            .unwrap()
            .split(";")
            .map(|s| s.trim().parse().unwrap())
            .collect();
        Ok(Self { id, sets })
    }
}

impl Day for Day2 {
    fn task1(&self, file: &std::path::PathBuf) {
        let games: Vec<Game> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let limits = SetSummary {
            red: Cube::Red(12),
            green: Cube::Green(13),
            blue: Cube::Blue(14),
        };

        let mut all = 0;
        for game in &games {
            all += game.id;
            for set in &game.sets {
                let summary: SetSummary = set.into();
                if summary.red > limits.red
                    || summary.green > limits.green
                    || summary.blue > limits.blue
                {
                    all -= game.id;
                    break;
                }
            }
        }
        println!("{}", all);
    }
    fn task2(&self, file: &std::path::PathBuf) {
        let games: Vec<Game> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let mut sum = 0;
        for game in &games {
            let summaries: Vec<SetSummary> = game.sets.iter().map(|s| s.into()).collect();
            let mut m: SetSummary = summaries
                .iter()
                .max_by_key(|&ss| ss.red)
                .unwrap()
                .red
                .into();
            m += summaries
                .iter()
                .max_by_key(|&ss| ss.green)
                .unwrap()
                .green
                .into();
            m += summaries
                .iter()
                .max_by_key(|&ss| ss.blue)
                .unwrap()
                .blue
                .into();
            sum += m.mult();
        }
        println!("{}", sum);
    }
}
