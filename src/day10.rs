use std::{collections::HashMap, fs};

use super::Day;

pub struct Day10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Ground,
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            'S' => Tile::Start,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Ground => '.',
                Tile::NS => '|',
                Tile::EW => '-',
                Tile::NE => 'L',
                Tile::NW => 'J',
                Tile::SW => '7',
                Tile::SE => 'F',
                Tile::Start => 'S',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl std::ops::AddAssign<&Direction> for (usize, usize) {
    fn add_assign(&mut self, rhs: &Direction) {
        if rhs == &Direction::North {
            self.0 -= 1;
        } else if rhs == &Direction::South {
            self.0 += 1;
        } else if rhs == &Direction::East {
            self.1 += 1;
        } else if rhs == &Direction::West {
            self.1 -= 1;
        }
    }
}

impl std::ops::Add<&Direction> for (usize, usize) {
    type Output = (usize, usize);
    fn add(self, rhs: &Direction) -> Self::Output {
        match rhs {
            Direction::North => (self.0 - 1, self.1),
            Direction::South => (self.0 + 1, self.1),
            Direction::East => (self.0, self.1 + 1),
            Direction::West => (self.0, self.1 - 1),
        }
    }
}

fn find_connection(tile: Tile, relationship: Direction) -> bool {
    match relationship {
        Direction::West => tile == Tile::EW || tile == Tile::NE || tile == Tile::SE,
        Direction::East => tile == Tile::EW || tile == Tile::NW || tile == Tile::SW,
        Direction::North => tile == Tile::NS || tile == Tile::SE || tile == Tile::SW,
        Direction::South => tile == Tile::NS || tile == Tile::NE || tile == Tile::NW,
    }
}

fn is_connected(start: Tile, end: Tile, relationship: Direction) -> bool {
    match relationship {
        Direction::West => {
            (end == Tile::EW || end == Tile::NE || end == Tile::SE)
                && (start == Tile::EW || start == Tile::NW || start == Tile::SW)
        }
        Direction::East => {
            (end == Tile::EW || end == Tile::NW || end == Tile::SW)
                && (start == Tile::EW || start == Tile::NE || start == Tile::SE)
        }
        Direction::North => {
            (end == Tile::NS || end == Tile::SE || end == Tile::SW)
                && (start == Tile::NS || start == Tile::NE || start == Tile::NW)
        }
        Direction::South => {
            (end == Tile::NS || end == Tile::NE || end == Tile::NW)
                && (start == Tile::NS || start == Tile::SE || start == Tile::SW)
        }
    }
}

fn get_surrounding(
    posn: (usize, usize),
    tiles: &[Vec<Tile>],
) -> (Option<Tile>, Option<Tile>, Option<Tile>, Option<Tile>) {
    let (r, c) = posn;
    let row = &tiles[r];
    let west = if c == 0 {
        None
    } else {
        row.get(c - 1).copied()
    };
    let east = row.get(c + 1).copied();
    let north = if r == 0 {
        None
    } else {
        tiles.get(r - 1).map(|row| &row[c]).copied()
    };
    let south = tiles.get(r + 1).map(|row| &row[c]).copied();
    (north, south, east, west)
}

fn where_next(posn: (usize, usize), tiles: &[Vec<Tile>]) -> Vec<Direction> {
    // Look around me
    let (r, c) = posn;
    let (north, south, east, west) = get_surrounding(posn, tiles);
    // I'll be connected to exactly two
    let mut connected = vec![];
    if let Some(west) = west {
        if is_connected(tiles[r][c], west, Direction::West) {
            connected.push(Direction::West);
        }
    }
    if let Some(east) = east {
        if is_connected(tiles[r][c], east, Direction::East) {
            connected.push(Direction::East);
        }
    }
    if let Some(north) = north {
        if is_connected(tiles[r][c], north, Direction::North) {
            connected.push(Direction::North);
        }
    }
    if let Some(south) = south {
        if is_connected(tiles[r][c], south, Direction::South) {
            connected.push(Direction::South);
        }
    }
    connected.sort();
    connected
}

fn traverse(start: (usize, usize), tiles: &[Vec<Tile>]) -> HashMap<(usize, usize), usize> {
    let mut counts = HashMap::from([(start, 0usize)]);
    let dirs = where_next(start, tiles);
    // Once from each direction

    for start_dir in dirs {
        let mut prev = start;
        let mut dir = start_dir;
        let mut seed = prev + &dir;
        let mut count: usize = 0;
        loop {
            count += 1;
            counts
                .entry(seed)
                .and_modify(|e| *e = *e.min(&mut count))
                .or_insert(count);
            dir = where_next(seed, tiles)
                .into_iter()
                .find(|d| (seed + d) != prev)
                .unwrap();
            prev = seed;
            seed += &dir;
            if prev == start {
                break;
            }
        }
    }
    counts
}

fn find_start(tiles: &[Vec<Tile>]) -> ((usize, usize), Tile) {
    for r in 0..tiles.len() {
        for c in 0..tiles[r].len() {
            if tiles[r][c] == Tile::Start {
                let (north, south, east, west) = get_surrounding((r, c), tiles);
                // I'll be connected to exactly two
                let mut connected = vec![];
                if let Some(west) = west {
                    if find_connection(west, Direction::West) {
                        connected.push(Direction::West);
                    }
                }
                if let Some(east) = east {
                    if find_connection(east, Direction::East) {
                        connected.push(Direction::East);
                    }
                }
                if let Some(north) = north {
                    if find_connection(north, Direction::North) {
                        connected.push(Direction::North);
                    }
                }
                if let Some(south) = south {
                    if find_connection(south, Direction::South) {
                        connected.push(Direction::South);
                    }
                }
                connected.sort();
                let result = match (connected[0], connected[1]) {
                    (Direction::East, Direction::West) => Tile::EW,
                    (Direction::South, Direction::East) => Tile::SE,
                    (Direction::South, Direction::West) => Tile::SW,
                    (Direction::North, Direction::East) => Tile::NE,
                    (Direction::North, Direction::West) => Tile::NW,
                    (Direction::North, Direction::South) => Tile::NS,
                    (_, _) => unreachable!(),
                };

                return ((r, c), result);
            }
        }
    }
    unreachable!()
}

impl Day for Day10 {
    fn task1(&self, file: &std::path::Path) {
        let mut tiles: Vec<Vec<Tile>> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.chars().map(std::convert::Into::into).collect())
            .collect();
        let ((r, c), t) = find_start(&tiles);
        tiles[r][c] = t;
        let counts = traverse((r, c), &tiles);
        println!("{}", counts.values().max().unwrap());
    }
    fn task2(&self, file: &std::path::Path) {
        let mut tiles: Vec<Vec<Tile>> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.chars().map(std::convert::Into::into).collect())
            .collect();
        let ((r, c), t) = find_start(&tiles);
        tiles[r][c] = t;
        let counts = traverse((r, c), &tiles);
        let path: Vec<_> = counts.keys().map(|&(r, c)| ((r, c), tiles[r][c])).collect();
        let grid = Grid::from(path);
        println!("{}", grid.num_contained());
    }
}

#[derive(Clone)]
pub struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl<I> From<I> for Grid
where
    I: IntoIterator<Item = ((usize, usize), Tile)>,
{
    fn from(value: I) -> Self {
        let path: Vec<_> = value.into_iter().collect();
        let rows = path.iter().map(|&((r, _), _)| r).max().unwrap() + 1;
        let cols = path.iter().map(|&((_, c), _)| c).max().unwrap() + 1;
        let mut grid = vec![vec![Tile::Ground; cols]; rows];
        for ((r, c), t) in path {
            grid[r][c] = t;
        }
        Grid { grid }
    }
}

impl Grid {
    #[must_use]
    pub fn num_contained(&self) -> usize {
        let mut count = 0;
        for r in 0..self.grid.len() {
            for c in 0..self.grid[r].len() {
                if self.grid[r][c] == Tile::Ground && self.is_inside((r, c)) {
                    count += 1;
                }
            }
        }
        count
    }

    #[must_use]
    pub fn is_inside(&self, posn: (usize, usize)) -> bool {
        let (r, c) = posn;
        self.grid[r][c] == Tile::Ground && self.raycast(posn) % 2 == 1
    }

    fn raycast(&self, posn: (usize, usize)) -> usize {
        let (row, col) = posn;
        let mut count = 0;
        for c in 0..col {
            if !matches!(
                self.grid[row][c],
                Tile::Ground | Tile::Start | Tile::EW | Tile::NE | Tile::NW
            ) {
                count += 1;
            }
        }
        count
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for r in 0..self.grid.len() {
            for c in 0..self.grid[r].len() {
                if self.grid[r][c] != Tile::Ground {
                    out.push('*');
                } else if self.is_inside((r, c)) {
                    out.push('I');
                } else {
                    out.push('.');
                }
            }
            out.push('\n');
        }
        f.write_str(&out)
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for t in row {
                write!(f, "{t}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
