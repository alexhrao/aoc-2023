use std::{collections::HashMap, fs, str::FromStr};

use super::Day;

use petgraph::algo::all_simple_paths;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;

pub struct Day23;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    const DIRS: [Direction; 4] = [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];
    pub fn diff(start: (usize, usize), end: (usize, usize)) -> Direction {
        if start.0 < end.0 {
            Direction::Down
        } else if start.0 > end.0 {
            Direction::Up
        } else if start.1 < end.1 {
            Direction::Right
        } else if start.1 > end.1 {
            Direction::Left
        } else {
            unreachable!()
        }
    }
    pub fn is_opposite(self, other: Direction) -> bool {
        match self {
            Direction::Down => other == Direction::Up,
            Direction::Up => other == Direction::Down,
            Direction::Left => other == Direction::Right,
            Direction::Right => other == Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Space {
    Path,
    Forest,
    Slope(Direction),
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Path,
            '#' => Space::Forest,
            s => Space::Slope(s.into()),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<Space>>,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<Space>> = s
            .lines()
            .map(|l| l.chars().map(std::convert::Into::into).collect())
            .collect();
        Ok(Self { map })
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for space in row {
                write!(
                    f,
                    "{}",
                    match space {
                        Space::Forest => "#",
                        Space::Path => ".",
                        Space::Slope(s) => match s {
                            Direction::Down => "v",
                            Direction::Up => "^",
                            Direction::Left => "<",
                            Direction::Right => ">",
                        },
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn remove_slopes(&mut self) {
        for row in &mut self.map {
            for t in row {
                if let Space::Slope(_) = t {
                    *t = Space::Path;
                }
            }
        }
    }
    pub fn graph_setup(&self) -> Graph<(usize, usize), ()> {
        // What do I want? A graph, directed, acyclic, then we can get longest path
        // Start, of course, by creating the graph
        let mut graph: Graph<(usize, usize), ()> = Graph::default();
        let nodes: HashMap<_, _> = self
            .map
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(c, s)| {
                        if s == &Space::Path {
                            Some(((r, c), graph.add_node((r, c))))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<((usize, usize), NodeIndex<u32>)>>()
                    .into_iter()
            })
            .collect();
        let (src, _) = nodes.iter().find(|(&(r, _), _)| r == 0).unwrap();
        self.add_edges(&mut graph, *src, &nodes);
        graph
    }
    pub fn add_edges(
        &self,
        graph: &mut Graph<(usize, usize), ()>,
        src: (usize, usize),
        nodes: &HashMap<(usize, usize), NodeIndex<u32>>,
    ) {
        for d in &Direction::DIRS {
            if let Some(dst) = self.step(src, *d) {
                // There exists a path from (sr, sc) -> (nr, nc)!
                let a = nodes[&src];
                let b = nodes[&dst];
                if !graph.contains_edge(a, b) {
                    graph.add_edge(nodes[&src], nodes[&dst], ());
                    // Explore!
                    self.add_edges(graph, dst, nodes);
                }
            }
        }
    }
    pub fn naive_explore(&self, path: &[(usize, usize)]) -> Vec<Vec<(usize, usize)>> {
        // We're starting at the tail
        let (sr, sc) = *path.last().unwrap();
        if sr == self.map.len() - 1 {
            return vec![path.to_vec()];
        }
        let mut out = vec![];
        for d in &Direction::DIRS {
            if let Some((nr, nc)) = self.step((sr, sc), *d) {
                if !path.contains(&(nr, nc)) {
                    let mut seed = path.to_vec();
                    seed.push((nr, nc));
                    // println!("{:?}", seed);
                    for np in self.naive_explore(&seed) {
                        out.push(np);
                    }
                }
            }
        }
        out
    }
    pub fn step(&self, start: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let (r, c) = start;
        if let Space::Slope(s) = self.map[r][c] {
            if s != dir {
                return None;
            }
        }
        let check_dir = move |end: (usize, usize)| -> Option<(usize, usize)> {
            let s = self.map[end.0][end.1];
            if let Space::Slope(d1) = s {
                let d2 = Direction::diff(start, end);
                if d1.is_opposite(d2) {
                    None
                } else {
                    Some(end)
                }
            } else if s == Space::Path {
                Some(end)
            } else {
                None
            }
        };
        match dir {
            Direction::Down => {
                if self.map.get(r + 1).map(|row| row[c]).is_some() {
                    check_dir((r + 1, c))
                } else {
                    None
                }
            }
            Direction::Up => {
                if r == 0 {
                    None
                } else {
                    check_dir((r - 1, c))
                }
            }
            Direction::Left => {
                if c == 0 {
                    None
                } else {
                    check_dir((r, c - 1))
                }
            }
            Direction::Right => {
                if self.map[r].get(c + 1).is_some() {
                    check_dir((r, c + 1))
                } else {
                    None
                }
            }
        }
    }
}

impl Day for Day23 {
    fn task1(&self, file: &std::path::Path) {
        let map: Map = fs::read_to_string(file).unwrap().parse().unwrap();
        let path = vec![(0, 1)];
        let paths = map.naive_explore(&path);
        let m = paths.iter().map(Vec::len).max().unwrap();
        println!("{}", m - 1);
    }
    fn task2(&self, file: &std::path::Path) {
        let mut map: Map = fs::read_to_string(file).unwrap().parse().unwrap();
        map.remove_slopes();
        let graph = map.graph_setup();
        let si = graph
            .node_references()
            .find_map(|(ni, (r, _))| if *r == 0 { Some(ni) } else { None })
            .unwrap();
        let di = graph
            .node_references()
            .find_map(|(ni, (r, _))| {
                if *r == (map.map.len() - 1) {
                    Some(ni)
                } else {
                    None
                }
            })
            .unwrap();
        let ways: Vec<_> = all_simple_paths::<Vec<_>, _>(&graph, si, di, 0, None).collect();
        // That took 12 minutes in release mode...
        let m = ways.iter().map(Vec::len).max().unwrap();
        println!("{}", m - 1);
    }
}
