use aoc_runner_derive::{aoc, aoc_generator};

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
        seed = out.last().unwrap();
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

#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(hists: &[Vec<isize>]) -> isize {
    hists
        .iter()
        .map(|h| diff(h))
        .map(build_out)
        .map(|d| *d.last().unwrap().last().unwrap())
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(hists: &[Vec<isize>]) -> isize {
    hists
        .iter()
        .map(|h| diff(h))
        .map(build_in)
        .map(|d| *d.last().unwrap().first().unwrap())
        .sum()
}
