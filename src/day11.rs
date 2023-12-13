use std::fs;

use super::Day;

pub struct Day11;

fn expand(galaxies: &[(usize, usize)], replace: usize) -> Vec<(usize, usize)> {
    let occ_rows: Vec<_> = galaxies.iter().map(|&g| g.0).collect();
    let occ_cols: Vec<_> = galaxies.iter().map(|&g| g.1).collect();
    let max_rows = *occ_rows.iter().max().unwrap();
    let max_cols = *occ_cols.iter().max().unwrap();
    let mut exp_rows: Vec<_> = (0..=max_rows).filter(|r| !occ_rows.contains(r)).collect();
    let mut exp_cols: Vec<_> = (0..=max_cols).filter(|c| !occ_cols.contains(c)).collect();
    exp_rows.sort();
    exp_cols.sort();

    let mut out = galaxies.to_vec();
    // Now, for each expanded row, add an extra row to anyone greater than me
    out.sort_by_key(|&(r, _)| r);
    for row in exp_rows.into_iter().rev() {
        for e in out.iter_mut().skip_while(|(r, _)| r < &row) {
            e.0 += replace - 1;
        }
    }
    // Now, for each expanded row, add an extra row to anyone greater than me
    out.sort_by_key(|&(_, c)| c);
    for col in exp_cols.into_iter().rev() {
        for e in out.iter_mut().skip_while(|(_, c)| c < &col) {
            e.1 += replace - 1;
        }
    }
    out
}

fn dist(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    (p1.0.max(p2.0) - p1.0.min(p2.0)) + (p1.1.max(p2.1) - p1.1.min(p2.1))
}

impl Day for Day11 {
    fn task1(&self, file: &std::path::Path) {
        let galaxies: Vec<_> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .enumerate()
            .flat_map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(c, ch)| if ch == '#' { Some((r, c)) } else { None })
            })
            .collect();

        let shifted = expand(&galaxies, 2);
        let mut dists = vec![];
        for g1 in 0..shifted.len() {
            for g2 in (g1 + 1)..shifted.len() {
                dists.push(dist(&shifted[g1], &shifted[g2]));
            }
        }
        println!("{}", dists.iter().sum::<usize>());
    }
    fn task2(&self, file: &std::path::Path) {
        let galaxies: Vec<_> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .enumerate()
            .flat_map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(c, ch)| if ch == '#' { Some((r, c)) } else { None })
            })
            .collect();
        println!("{}", galaxies.len());

        let shifted = expand(&galaxies, 1000000);
        println!("{}", shifted.len());
        let mut dists = vec![];
        for g1 in 0..shifted.len() {
            for g2 in (g1 + 1)..shifted.len() {
                dists.push(dist(&shifted[g1], &shifted[g2]));
            }
        }
        println!("{}", dists.iter().sum::<usize>());
    }
}
