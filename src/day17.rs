use std::{collections::HashMap, fs};

use super::Day;
use petgraph::algo::dijkstra;
use petgraph::Graph;

pub struct Day17;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn next(&self, posn: (usize, usize), dims: (usize, usize)) -> Option<(usize, usize)> {
        let (r, c) = posn;
        let (rows, cols) = dims;
        match self {
            Direction::Up => {
                if r == 0 {
                    None
                } else {
                    Some((r - 1, c))
                }
            }
            Direction::Down => {
                if r == (rows - 1) {
                    None
                } else {
                    Some((r + 1, c))
                }
            }
            Direction::Left => {
                if c == 0 {
                    None
                } else {
                    Some((r, c - 1))
                }
            }
            Direction::Right => {
                if c == (cols - 1) {
                    None
                } else {
                    Some((r, c + 1))
                }
            }
        }
    }
    pub fn orthogonals(&self) -> [Direction; 2] {
        match self {
            &Direction::Up | &Direction::Down => [Direction::Left, Direction::Right],
            &Direction::Left | &Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Block {
    posn: (usize, usize),
    dir: Direction,
    count: usize,
}

impl Day for Day17 {
    fn task1(&self, file: &std::path::Path) {
        let blocks: Vec<Vec<usize>> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect();
        let mut graph = Graph::<_, _>::new();
        let dirs = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        let mut nodes = Vec::with_capacity(blocks.len());
        for (r, block_row) in blocks.iter().enumerate() {
            let mut row = Vec::with_capacity(blocks[r].len());
            for c in 0..block_row.len() {
                let mut col = HashMap::with_capacity(dirs.len());
                for d in &dirs {
                    let mut stack = Vec::with_capacity(3);
                    // Could be one, two, or three
                    for i in 1..=3 {
                        let idx = graph.add_node(Block {
                            count: i,
                            dir: *d,
                            posn: (r, c),
                        });
                        stack.push(idx);
                    }
                    col.insert(*d, stack);
                }
                row.push(col);
            }
            nodes.push(row);
        }

        let dims = (blocks.len(), blocks[0].len());

        // Now go through to make edges
        for r in 0..blocks.len() {
            for c in 0..blocks[r].len() {
                for d in &dirs {
                    for i in 1..=3 {
                        if i != 3 {
                            // I'm not at the top. I can connect to i + 1,
                            // with the weight of the NEXT one in this same direction...
                            // at least, if I'm not at the edge
                            if let Some((nr, nc)) = d.next((r, c), dims) {
                                // wt is how much it costs to get to the next one.
                                let wt = blocks[nr][nc];
                                graph.add_edge(nodes[r][c][d][i - 1], nodes[nr][nc][d][i], wt);
                            }
                        }
                        // I can always go the orthogonal directions...
                        for ortho in d.orthogonals() {
                            // ... or can I? Check!
                            if let Some((nr, nc)) = ortho.next((r, c), dims) {
                                let wt = blocks[nr][nc];
                                graph.add_edge(nodes[r][c][d][i - 1], nodes[nr][nc][&ortho][0], wt);
                            }
                        }
                    }
                }
            }
        }
        let mut wts = vec![];
        for start_dir in &[Direction::Right, Direction::Down] {
            let dists = dijkstra(&graph, nodes[0][0][start_dir][0], None, |e| *e.weight());
            wts.extend(dirs.iter().flat_map(|d| {
                (0..3).filter_map(|i| dists.get(&nodes[dims.0 - 1][dims.1 - 1][d][i]).copied())
            }));
        }
        println!("{:?}", wts.into_iter().min().unwrap());
    }
    fn task2(&self, _file: &std::path::Path) {}
}
