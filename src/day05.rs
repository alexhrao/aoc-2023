use rayon::prelude::*;
use std::{fmt::Display, fs, str::FromStr};

use super::Day;

pub struct Day05;

#[derive(Debug, Clone, Copy)]
struct Range {
    dst: usize,
    src: usize,
    len: usize,
}

impl Range {
    pub fn contains_seed(&self, seed: usize) -> bool {
        seed >= self.src && seed < (self.src + self.len)
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitter = s.split_whitespace();
        let dst = splitter.next().unwrap().parse().unwrap();
        let src = splitter.next().unwrap().parse().unwrap();
        let len = splitter.next().unwrap().parse().unwrap();
        Ok(Range { dst, src, len })
    }
}
#[derive(Debug, Clone)]
struct Map {
    src: String,
    dst: String,
    ranges: Vec<Range>,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut name = lines.next().unwrap().split('-');
        let src = name.next().unwrap().to_string();
        let dst = name
            .nth(1)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .to_string();
        let ranges = lines
            .filter(|l| !l.is_empty())
            .map(|l| l.parse().unwrap())
            .collect();
        Ok(Map { src, dst, ranges })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}: ", self.src, self.dst)?;
        let ranges: Vec<_> = self
            .ranges
            .iter()
            .map(|r| format!("{} -> {} ({})", r.src, r.dst, r.len))
            .collect();
        write!(f, "{}", ranges.join("; "))
    }
}

impl Map {
    pub fn translate(&self, seed: usize) -> usize {
        for r in &self.ranges {
            if r.contains_seed(seed) {
                return r.dst + (seed - r.src);
            }
        }
        seed
    }
}

fn traverse(seed: usize, maps: &[Map]) -> usize {
    let mut seed = seed;
    for map in maps {
        seed = map.translate(seed);
    }
    seed
}

impl Day for Day05 {
    fn task1(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let mut lines = backing.lines();
        let seed_line = lines.next().unwrap();
        let seeds: Vec<usize> = seed_line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        let mut maps: Vec<Map> = vec![];
        let mut buf = vec![];
        for line in lines.skip(1) {
            if line.is_empty() {
                // snip and get a map
                maps.push(buf.join("\n").parse().unwrap());
                buf.clear();
            } else {
                buf.push(line);
            }
        }
        maps.push(buf.join("\n").parse().unwrap());
        // for map in maps {
        //     println!("{}", map);
        // }
        let seeds: Vec<_> = seeds.into_iter().map(|s| traverse(s, &maps)).collect();
        println!("{:?}", seeds.iter().min().unwrap());
    }
    fn task2(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let mut lines = backing.lines();
        let seed_line = lines.next().unwrap();
        let mut maps: Vec<Map> = vec![];
        let mut buf = vec![];
        for line in lines.skip(1) {
            if line.is_empty() {
                // snip and get a map
                maps.push(buf.join("\n").parse().unwrap());
                buf.clear();
            } else {
                buf.push(line);
            }
        }
        maps.push(buf.join("\n").parse().unwrap());
        let seeds: Vec<usize> = seed_line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        let seeds: Vec<_> = seeds
            .chunks_exact(2)
            .map(|ss| (ss[0]..(ss[0] + ss[1])).into_par_iter())
            .collect();
        let mut out = vec![];
        for rng in seeds {
            out.par_extend(rng.map(|s| traverse(s, &maps)));
        }
        println!("{}", out.iter().min().unwrap());
        // let seeds: Vec<_> = seeds
        //     .into_iter()
        //     .map(|s| traverse(s, &maps))
        //     .collect();
        // println!("{:?}", seeds.iter().min().unwrap());
    }
}
