use std::fs;

use super::Day;

pub struct Day3;

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
struct Schematic {
    rows: Vec<Vec<char>>,
}

impl Schematic {
    pub fn from_rows(rows: Vec<Vec<char>>) -> Schematic {
        Schematic { rows }
    }
    pub fn extract(&self) -> (Vec<Part>, Vec<Symbol>) {
        let mut parts = vec![];
        let mut symbols = vec![];
        for (r, row) in self.rows.iter().enumerate() {
            let mut start = None;
            let mut stop = None;
            for (c, &ch) in row.iter().enumerate() {
                if ch >= '0' && ch <= '9' {
                    if start.is_none() {
                        start = Some(c);
                    }
                    stop = Some(c);
                } else if let Some(end) = stop {
                    let num: String = row[start.unwrap()..=end].into_iter().collect();
                    parts.push(Part { row: r, start: start.unwrap(), stop: end, num: num.parse().unwrap() });
                    start = None;
                    stop = None;
                } else {
                    
                }
                if (ch < '0' || ch > '9') && ch != '.' {
                    symbols.push(Symbol { row: r, col: c, c: ch })
                }
            }
            if let Some(end) = stop {
                let num: String = row[start.unwrap()..=end].into_iter().collect();
                parts.push(Part { row: r, start: start.unwrap(), stop: end, num: num.parse().unwrap() });
            }
        }
        (parts, symbols)
    }
}

impl Day for Day3 {
    fn task1(&self, file: &std::path::PathBuf) {
        let s = Schematic::from_rows(fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.chars().collect())
            .collect());
        let (parts, symbols) = s.extract();
        let parts: Vec<Part> = parts.into_iter()
            .filter(|p| {
                // If any symbol is close to us, keep it. We define "close" as:
                // 1. Same row, but s.col = start - 1 or stop + 1
                // 2. 1 row away, (start - 1) <= s.col <= (stop + 1)
                for s in symbols.iter().skip_while(|s| p.row > 0 && s.row < (p.row - 1)) {
                    if s.row > (p.row + 1) {
                        break;
                    }
                    if s.col >= (p.start.max(1) - 1) && s.col <= (p.stop + 1) {
                        return true;
                    }
                }
                false
            }).collect();
        let total: usize = parts.iter().map(|&p| p.num).sum();
        println!("{:?}", total);
    }
    fn task2(&self, file: &std::path::PathBuf) {
        let s = Schematic::from_rows(fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.chars().collect())
            .collect());
        let (parts, symbols) = s.extract();
        let total: usize = symbols.into_iter()
            .filter_map(|s| {
                if s.c != '*' {
                    return None;
                }
                // Look through the parts.
                let mut these_parts = vec![];
                for p in parts.iter().skip_while(|p| s.row > 0 && (p.row < (s.row - 1))) {
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
                // Some(these_parts)
            }).sum();
        println!("{}", total);
    }
}