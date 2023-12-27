use std::{collections::HashMap, fmt::Display, fs, str::FromStr};

use super::Day;

pub struct Day14;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Cube,
    Sphere,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Empty,
            '#' => Space::Cube,
            'O' => Space::Sphere,
            _ => unreachable!(),
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::Empty => '.',
                Space::Cube => '#',
                Space::Sphere => 'O',
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    grid: Vec<Vec<Space>>,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|r| r.chars().map(std::convert::Into::into).collect())
            .collect();
        Ok(Grid { grid })
    }
}

impl Grid {
    fn transpose(&mut self) {
        self.grid = if let Some(head) = self.grid.first() {
            let mut out = vec![vec![Space::Empty; self.grid.len()]; head.len()];
            for r in 0..self.grid.len() {
                for (c, item) in out.iter_mut().enumerate().take(self.grid[r].len()) {
                    item[r] = self.grid[r][c];
                }
            }
            out
        } else {
            vec![]
        };
    }

    pub fn tilt_north(&mut self) {
        self.transpose();
        for c in 0..self.grid.len() {
            let col = &mut self.grid[c];
            for r in 0..col.len() {
                if col[r] == Space::Sphere {
                    // It could roll. Find the first entry above me that
                    // isn't occupied
                    let idx = col.iter().take(r).enumerate().rev().find_map(|(i, s)| {
                        if s == &Space::Empty {
                            None
                        } else {
                            Some(i)
                        }
                    });
                    // Clear mine, but then replace with idx
                    let idx = if let Some(idx) = idx { idx + 1 } else { 0 };
                    col[r] = Space::Empty;
                    col[idx] = Space::Sphere;
                }
            }
        }
        self.transpose();
    }
    pub fn tilt_south(&mut self) {
        self.grid = self.grid.iter().cloned().rev().collect();
        self.tilt_north();
        self.grid = self.grid.iter().cloned().rev().collect();
    }
    pub fn tilt_west(&mut self) {
        self.transpose();
        self.tilt_north();
        self.transpose();
    }
    pub fn tilt_east(&mut self) {
        self.transpose();
        self.grid = self.grid.iter().cloned().rev().collect();
        self.tilt_north();
        self.grid = self.grid.iter().cloned().rev().collect();
        self.transpose();
    }
    pub fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
    pub fn north_load(&self) -> usize {
        self.grid
            .iter()
            .rev()
            .enumerate()
            .map(|(r, row)| row.iter().filter(|&&c| c == Space::Sphere).count() * (r + 1))
            .sum::<usize>()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{col}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Day for Day14 {
    fn task1(&self, file: &std::path::Path) {
        let mut grid: Grid = fs::read_to_string(file).unwrap().parse().unwrap();
        grid.tilt_north();
        println!("{}", grid.north_load());
    }
    fn task2(&self, file: &std::path::Path) {
        let mut grid: Grid = fs::read_to_string(file).unwrap().parse().unwrap();
        let mut past = HashMap::new();

        for i in 0..1_000_000_000 {
            if let Some(idx) = past.get(&grid) {
                let modulus = i - *idx;
                let oper = 1_000_000_000 - *idx;
                for _ in 0..(oper % modulus) {
                    grid.cycle();
                }
                break;
            }
            past.insert(grid.clone(), i);
            grid.cycle();
        }
        println!("{}", grid.north_load());
    }
}
