use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet};

use petgraph::algo::dijkstra;
use petgraph::Graph;

const T1_MAX_STRAIGHT: usize = 3;
const T2_MIN_STRAIGHT: usize = 4;
const T2_MAX_STRAIGHT: usize = 10;
const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
const START_DIRS: [Direction; 2] = [Direction::Down, Direction::Right];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn next(self, posn: (usize, usize), dims: (usize, usize)) -> Option<(usize, usize)> {
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
    pub fn orthogonals(self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Block {
    posn: (usize, usize),
    dir: Direction,
    count: usize,
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let blocks: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();
    let mut graph = Graph::<_, _>::new();
    let mut nodes = Vec::with_capacity(blocks.len());
    for (r, block_row) in blocks.iter().enumerate() {
        let mut row = Vec::with_capacity(blocks[r].len());
        for c in 0..block_row.len() {
            let mut col = HashMap::with_capacity(DIRS.len());
            for d in &DIRS {
                let mut stack = Vec::with_capacity(3);
                // Could be one, two, or three
                for i in 1..=T1_MAX_STRAIGHT {
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
    let nodes = nodes;
    let dims = (blocks.len(), blocks[0].len());

    // Now go through to make edges
    for r in 0..blocks.len() {
        for c in 0..blocks[r].len() {
            for d in &DIRS {
                for i in 1..T1_MAX_STRAIGHT {
                    // I'm not at the top. I can connect to i + 1,
                    // with the weight of the NEXT one in this same direction...
                    // at least, if I'm not at the edge
                    if let Some((nr, nc)) = d.next((r, c), dims) {
                        // wt is how much it costs to get to the next one.
                        let wt = blocks[nr][nc];
                        graph.add_edge(nodes[r][c][d][i - 1], nodes[nr][nc][d][i], wt);
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
                for ortho in d.orthogonals() {
                    // ... or can I? Check!
                    if let Some((nr, nc)) = ortho.next((r, c), dims) {
                        let wt = blocks[nr][nc];
                        graph.add_edge(
                            nodes[r][c][d].last().copied().unwrap(),
                            nodes[nr][nc][&ortho][0],
                            wt,
                        );
                    }
                }
            }
        }
    }
    let graph = graph;

    let end_nodes: HashSet<_> = nodes
        .last()
        .unwrap()
        .last()
        .unwrap()
        .values()
        .flatten()
        .copied()
        .collect();

    nodes
        .first()
        .unwrap()
        .first()
        .unwrap()
        .iter()
        .filter_map(|(d, idx)| {
            if START_DIRS.contains(d) {
                Some(idx[0])
            } else {
                None
            }
        })
        .flat_map(|n| {
            dijkstra(&graph, n, None, |e| *e.weight())
                .into_iter()
                .filter_map(|(n_idx, cost)| {
                    if end_nodes.contains(&n_idx) {
                        Some(cost)
                    } else {
                        None
                    }
                })
        })
        .min()
        .unwrap()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let blocks: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();
    let mut graph = Graph::<_, _>::new();
    let mut nodes = Vec::with_capacity(blocks.len());
    for (r, block_row) in blocks.iter().enumerate() {
        let mut row = Vec::with_capacity(blocks[r].len());
        for c in 0..block_row.len() {
            let mut col = HashMap::with_capacity(DIRS.len());
            for d in &DIRS {
                let mut stack = Vec::with_capacity(3);
                for i in 1..=10 {
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
            for d in &DIRS {
                for i in 1..T2_MIN_STRAIGHT {
                    // HAVE to go in the same direction!
                    if let Some((nr, nc)) = d.next((r, c), dims) {
                        let wt = blocks[nr][nc];
                        graph.add_edge(nodes[r][c][d][i - 1], nodes[nr][nc][d][i], wt);
                    }
                }
                for i in T2_MIN_STRAIGHT..T2_MAX_STRAIGHT {
                    // I'm not at the top. I can connect to i + 1,
                    // with the weight of the NEXT one in this same direction...
                    // at least, if I'm not at the edge
                    if let Some((nr, nc)) = d.next((r, c), dims) {
                        // wt is how much it costs to get to the next one.
                        let wt = blocks[nr][nc];
                        graph.add_edge(nodes[r][c][d][i - 1], nodes[nr][nc][d][i], wt);
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
                for ortho in d.orthogonals() {
                    // ... or can I? Check!
                    if let Some((nr, nc)) = ortho.next((r, c), dims) {
                        let wt = blocks[nr][nc];
                        graph.add_edge(
                            nodes[r][c][d].last().copied().unwrap(),
                            nodes[nr][nc][&ortho][0],
                            wt,
                        );
                    }
                }
            }
        }
    }
    let graph = graph;
    let end_nodes: HashSet<_> = nodes[nodes.len() - 1][nodes.len() - 1]
        .values()
        .flatten()
        .copied()
        .collect();

    nodes[0][0]
        .iter()
        .filter_map(|(d, idx)| {
            if START_DIRS.contains(d) {
                Some(idx[0])
            } else {
                None
            }
        })
        .flat_map(|n| {
            dijkstra(&graph, n, None, |e| *e.weight())
                .into_iter()
                .filter_map(|(n_idx, cost)| {
                    if end_nodes.contains(&n_idx) {
                        Some(cost)
                    } else {
                        None
                    }
                })
        })
        .min()
        .unwrap()
}
