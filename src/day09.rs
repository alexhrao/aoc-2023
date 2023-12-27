use std::fs;

use super::Day;

pub struct Day09;

fn diff(hist: &[isize]) -> Vec<Vec<isize>> {
    let mut out = vec![hist.to_vec()];
    let mut seed = hist;
    loop {
        let diff: Vec<_> = seed.windows(2).map(|w| w[1] - w[0]).collect();
        let done = diff.iter().all(|&d| d == 0);
        out.push(diff);
        if done {
            break out;
        }
        seed = &out.last().unwrap();
    }
}

fn build_out(diffs: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut diffs = diffs;
    diffs.reverse();
    diffs[0].push(0);
    let len = diffs.len();
    for d in 1..len {
        let tmp = diffs[d].last().unwrap() + diffs[d - 1].last().unwrap();
        diffs[d].push(tmp);
    }
    diffs
}

fn build_in(diffs: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut diffs = diffs;
    diffs.reverse();
    diffs[0].insert(0, 0);
    let len = diffs.len();
    for d in 1..len {
        let tmp = diffs[d].first().unwrap() - diffs[d - 1].first().unwrap();
        diffs[d].insert(0, tmp);
    }
    diffs
}

impl Day for Day09 {
    fn task1(&self, file: &std::path::Path) {
        let hists: Vec<Vec<isize>> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
            .collect();
        let total = hists
            .iter()
            .map(|h| diff(h))
            .map(build_out)
            .map(|d| *d.last().unwrap().last().unwrap())
            .sum::<isize>();
        println!("{total:?}");
    }
    fn task2(&self, file: &std::path::Path) {
        let hists: Vec<Vec<isize>> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
            .collect();
        let total = hists
            .iter()
            .map(|h| diff(h))
            .map(build_in)
            .map(|d| *d.last().unwrap().first().unwrap())
            .sum::<isize>();
        println!("{total:?}");
    }
}
