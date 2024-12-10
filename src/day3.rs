use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
struct Part {
    row: usize,
    start: usize,
    stop: usize,
    num: usize,
}
#[derive(Debug, Clone, Copy)]
struct Symbol {
    row: usize,
    col: usize,
    c: char,
}

#[derive(Debug, Clone)]
pub struct Schematic {
    rows: Vec<Vec<char>>,
}

impl Schematic {
    pub fn from_rows(rows: Vec<Vec<char>>) -> Schematic {
        Schematic { rows }
    }
    fn extract(&self) -> (Vec<Part>, Vec<Symbol>) {
        let mut parts = vec![];
        let mut symbols = vec![];
        for (r, row) in self.rows.iter().enumerate() {
            let mut start = None;
            let mut stop = None;
            for (c, &ch) in row.iter().enumerate() {
                if ch.is_ascii_digit() {
                    if start.is_none() {
                        start = Some(c);
                    }
                    stop = Some(c);
                } else if let Some(end) = stop {
                    let num: String = row[start.unwrap()..=end].iter().collect();
                    parts.push(Part {
                        row: r,
                        start: start.unwrap(),
                        stop: end,
                        num: num.parse().unwrap(),
                    });
                    start = None;
                    stop = None;
                }
                if ch.is_ascii_digit() && ch != '.' {
                    symbols.push(Symbol {
                        row: r,
                        col: c,
                        c: ch,
                    });
                }
            }
            if let Some(end) = stop {
                let num: String = row[start.unwrap()..=end].iter().collect();
                parts.push(Part {
                    row: r,
                    start: start.unwrap(),
                    stop: end,
                    num: num.parse().unwrap(),
                });
            }
        }
        (parts, symbols)
    }
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Schematic {
    Schematic::from_rows(input.lines().map(|l| l.chars().collect()).collect())
}

#[aoc(day3, part1)]
pub fn part1(s: &Schematic) -> usize {
    let (parts, symbols) = s.extract();
    let parts: Vec<Part> = parts
        .into_iter()
        .filter(|p| {
            // If any symbol is close to us, keep it. We define "close" as:
            // 1. Same row, but s.col = start - 1 or stop + 1
            // 2. 1 row away, (start - 1) <= s.col <= (stop + 1)
            for s in symbols
                .iter()
                .skip_while(|s| p.row > 0 && s.row < (p.row - 1))
            {
                if s.row > (p.row + 1) {
                    break;
                }
                if s.col >= (p.start.max(1) - 1) && s.col <= (p.stop + 1) {
                    return true;
                }
            }
            false
        })
        .collect();
    parts.iter().map(|&p| p.num).sum()
}

#[aoc(day3, part2)]
pub fn part2(s: &Schematic) -> usize {
    let (parts, symbols) = s.extract();
    symbols
        .into_iter()
        .filter_map(|s| {
            if s.c != '*' {
                return None;
            }
            // Look through the parts.
            let mut these_parts = vec![];
            for p in parts
                .iter()
                .skip_while(|p| s.row > 0 && (p.row < (s.row - 1)))
            {
                // Same check
                if p.row > (s.row + 1) {
                    break;
                }
                if s.col >= (p.start.max(1) - 1) && s.col <= (p.stop + 1) {
                    these_parts.push(*p);
                }
            }
            if these_parts.len() == 2 {
                Some(these_parts[0].num * these_parts[1].num)
            } else {
                None
            }
        })
        .sum()
}
