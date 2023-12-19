use std::fs;

use super::Day;
use petgraph::Graph;

pub struct Day17;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Block {
    dir: Direction,
    count: usize,
}

impl Day for Day17 {
    fn task1(&self, file: &std::path::Path) {
        let blocks: Vec<Vec<u8>> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect();
        let nodes = Graph::<Block, ()>::new();
        let dirs = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        // for r in 0..blocks.len() {
        //     for c in 0..blocks[r].len() {
        //         for d in &dirs {
        //             // For count 1 and 2,
        //         }
        //     }
        // }
        for d in dirs {
            let b = Block { count: 1, dir: d };
            println!("{:?}", b);
        }
        println!("{:?}", nodes);
        println!("{:?}", blocks);
    }
    fn task2(&self, _file: &std::path::Path) {}
}
