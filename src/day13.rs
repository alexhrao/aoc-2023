use std::fs;

use super::Day;
pub struct Day13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reflection {
    Horizontal(usize, usize),
    Vertical(usize, usize),
}

fn find_reflection(elements: &[&str]) -> Option<(usize, usize)> {
    for mid in 1..elements.len() {
        // Work our way out. As long as they match, we should be good
        let (left, right) = elements.split_at(mid);
        let result = left.iter().rev().zip(right.iter()).all(|(l, r)| l == r);
        if result {
            return Some((mid, left.len().min(right.len())));
        }
    }
    None
}

fn find_all_reflections<'a>(elements: &'a [&'a str]) -> impl Iterator<Item = (usize, usize)> + 'a {
    (1..elements.len()).filter_map(|mid| {
        // Work our way out. As long as they match, we should be good
        let (left, right) = elements.split_at(mid);
        let result = left.iter().rev().zip(right.iter()).all(|(l, r)| l == r);
        if result {
            return Some((mid, left.len().min(right.len())));
        }
        None
    })
}

fn solve_puzzle(puzzle: &str) -> Option<Reflection> {
    let rows: Vec<&str> = puzzle.lines().collect();
    let mut cols = vec![String::new(); rows[0].len()];
    for row in &rows {
        for (c, ch) in row.chars().enumerate() {
            cols[c].push(ch);
        }
    }
    let cols: Vec<&str> = cols.iter().map(|x| &**x).collect();

    if let Some(r) = find_reflection(&rows) {
        Some(Reflection::Horizontal(r.0, r.1))
    } else {
        find_reflection(&cols).map(|c| Reflection::Vertical(c.0, c.1))
    }
}

fn solve_puzzle_again(puzzle: &str, prev: &Reflection) -> Option<Reflection> {
    let rows: Vec<&str> = puzzle.lines().collect();
    let mut cols = vec![String::new(); rows[0].len()];
    for row in &rows {
        for (c, ch) in row.chars().enumerate() {
            cols[c].push(ch);
        }
    }
    let cols: Vec<&str> = cols.iter().map(|x| &**x).collect();
    // println!("{:?}", rows);
    // println!("{:?}", cols);
    let out = find_all_reflections(&rows)
        .map(|r| Reflection::Horizontal(r.0, r.1))
        .chain(find_all_reflections(&cols).map(|c| Reflection::Vertical(c.0, c.1)))
        .find(|r| r != prev);
    out
}

fn try_smudges(puzzle: &str) -> Reflection {
    // Get the original
    let orig = solve_puzzle(puzzle).unwrap();
    let out = puzzle.char_indices().find_map(|(c, ch)| {
        let mut changed = String::from(puzzle);
        if ch == '.' {
            changed.replace_range(c..=c, "#");
        } else if ch == '#' {
            changed.replace_range(c..=c, ".");
        } else {
            return None;
        }
        solve_puzzle_again(&changed, &orig)
    });
    out.unwrap()
}

impl Day for Day13 {
    fn task1(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let puzzles = backing.split("\n\n");
        let soln = puzzles
            .map(solve_puzzle)
            .map(Option::unwrap)
            .map(|r| match r {
                Reflection::Horizontal(r, _) => r * 100,
                Reflection::Vertical(c, _) => c,
            })
            .sum::<usize>();
        println!("{soln}");
    }
    fn task2(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let puzzles = backing.split("\n\n");
        let soln = puzzles
            .map(try_smudges)
            .map(|r| match r {
                Reflection::Horizontal(r, _) => r * 100,
                Reflection::Vertical(c, _) => c,
            })
            .sum::<usize>();
        println!("{soln}");
    }
}
