use super::Day;
use rayon::prelude::*;
use std::{collections::HashSet, fs};

pub struct Day16;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    AngleUp,
    AngleDown,
    SplitVertical,
    SplitHorizontal,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::AngleUp,
            '\\' => Tile::AngleDown,
            '|' => Tile::SplitVertical,
            '-' => Tile::SplitHorizontal,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn add(&self, posn: (usize, usize), tiles: &[Vec<Tile>]) -> Option<(usize, usize)> {
        if self == &Direction::Up {
            if posn.0 == 0 {
                None
            } else {
                Some((posn.0 - 1, posn.1))
            }
        } else if self == &Direction::Down {
            if posn.0 == (tiles.len() - 1) {
                None
            } else {
                Some((posn.0 + 1, posn.1))
            }
        } else if self == &Direction::Left {
            if posn.1 == 0 {
                None
            } else {
                Some((posn.0, posn.1 - 1))
            }
        } else if self == &Direction::Right {
            if posn.1 == (tiles[0].len() - 1) {
                None
            } else {
                Some((posn.0, posn.1 + 1))
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Ray {
    posn: (usize, usize),
    dir: Direction,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            posn: (0, 0),
            dir: Direction::Right,
        }
    }
}
fn get_energized(
    seed: &Ray,
    tiles: &[Vec<Tile>],
    memo: &mut HashSet<Ray>,
) -> HashSet<(usize, usize)> {
    // visited != memo because memo has to keep in account direction
    let mut visited = HashSet::new();
    let mut ray = *seed;
    loop {
        if memo.contains(&ray) {
            break visited;
        }
        visited.insert(ray.posn);
        memo.insert(ray);
        let (r, c) = ray.posn;
        let tile = &tiles[r][c];
        match (tile, ray.dir) {
            (Tile::Empty, _)
            | (Tile::SplitHorizontal, Direction::Left | Direction::Right)
            | (Tile::SplitVertical, Direction::Up | Direction::Down) => {
                if let Some(tmp) = ray.dir.add(ray.posn, tiles) {
                    ray.posn = tmp;
                } else {
                    break visited;
                }
            }
            (Tile::AngleDown, Direction::Right) | (Tile::AngleUp, Direction::Left) => {
                ray.dir = Direction::Down;
                if let Some(tmp) = ray.dir.add(ray.posn, tiles) {
                    ray.posn = tmp;
                } else {
                    break visited;
                }
            }
            (Tile::AngleDown, Direction::Up) | (Tile::AngleUp, Direction::Down) => {
                ray.dir = Direction::Left;
                if let Some(tmp) = ray.dir.add(ray.posn, tiles) {
                    ray.posn = tmp;
                } else {
                    break visited;
                }
            }
            (Tile::AngleDown, Direction::Left) | (Tile::AngleUp, Direction::Right) => {
                ray.dir = Direction::Up;
                if let Some(tmp) = ray.dir.add(ray.posn, tiles) {
                    ray.posn = tmp;
                } else {
                    break visited;
                }
            }
            (Tile::AngleDown, Direction::Down) | (Tile::AngleUp, Direction::Up) => {
                ray.dir = Direction::Right;
                if let Some(tmp) = ray.dir.add(ray.posn, tiles) {
                    ray.posn = tmp;
                } else {
                    break visited;
                }
            }
            (Tile::SplitVertical, Direction::Left | Direction::Right) => {
                let ray_up = Ray {
                    dir: Direction::Up,
                    posn: ray.posn,
                };
                let ray_down = Ray {
                    dir: Direction::Down,
                    posn: ray.posn,
                };
                visited.extend(get_energized(&ray_up, tiles, memo));
                visited.extend(get_energized(&ray_down, tiles, memo));
                break visited;
            }
            (Tile::SplitHorizontal, Direction::Up | Direction::Down) => {
                let ray_left = Ray {
                    dir: Direction::Left,
                    posn: ray.posn,
                };
                let ray_right = Ray {
                    dir: Direction::Right,
                    posn: ray.posn,
                };
                visited.extend(get_energized(&ray_left, tiles, memo));
                visited.extend(get_energized(&ray_right, tiles, memo));
                break visited;
            }
        }
    }
}

impl Day for Day16 {
    fn task1(&self, file: &std::path::Path) {
        let tiles: Vec<Vec<Tile>> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();
        let mut memo = HashSet::new();
        println!(
            "{}",
            get_energized(&Ray::default(), &tiles, &mut memo).len()
        );
    }
    fn task2(&self, file: &std::path::Path) {
        let tiles: Vec<Vec<Tile>> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();
        let total = (0..tiles.len())
            .flat_map(|c| {
                [
                    Ray {
                        posn: (0, c),
                        dir: Direction::Down,
                    },
                    Ray {
                        posn: (tiles.len() - 1, c),
                        dir: Direction::Up,
                    },
                ]
            })
            .chain((0..tiles[0].len() - 1).flat_map(|r| {
                [
                    Ray {
                        posn: (r, 0),
                        dir: Direction::Right,
                    },
                    Ray {
                        posn: (r, tiles[0].len() - 1),
                        dir: Direction::Left,
                    },
                ]
            }))
            .par_bridge()
            .map(|r| get_energized(&r, &tiles, &mut HashSet::new()).len())
            .max()
            .unwrap();
        println!("{}", total);
    }
}
