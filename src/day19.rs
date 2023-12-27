use std::{collections::HashMap, fmt::Display, fs, str::FromStr};

use super::Day;
use regex::Regex;

pub struct Day19;

const MIN: usize = 1;
const MAX: usize = 4000;
const ACCEPTED: &str = "A";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Metric {
    X,
    M,
    A,
    S,
}

impl FromStr for Metric {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Metric::X,
            "m" => Metric::M,
            "a" => Metric::A,
            "s" => Metric::S,
            _ => unreachable!(),
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Comparator {
    LessThan,
    GreaterThan,
}
impl FromStr for Comparator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "<" => Comparator::LessThan,
            ">" => Comparator::GreaterThan,
            _ => unreachable!(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rule<'a> {
    field: Metric,
    op: Comparator,
    val: usize,
    dst: &'a str,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        let re = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
        if let Some(caps) = re.captures(s) {
            let field = caps.get(1).unwrap().as_str().parse().unwrap();
            let op = caps.get(2).unwrap().as_str().parse().unwrap();
            let val = caps.get(3).unwrap().as_str().parse().unwrap();
            let dst = caps.get(4).unwrap().as_str();
            Self {
                field,
                op,
                val,
                dst,
            }
        } else {
            unreachable!()
        }
    }
}

impl<'a> Rule<'a> {
    pub fn check_part(&self, part: &Part) -> Option<&str> {
        let val = part[self.field];
        let result = match self.op {
            Comparator::GreaterThan => val > self.val,
            Comparator::LessThan => val < self.val,
        };
        if result {
            Some(self.dst)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    fallback: &'a str,
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let mut split = value.split(|c| c == '{' || c == '}');
        let name = split.next().unwrap();
        let mut rules: Vec<&str> = split.next().unwrap().split(',').collect();
        let fallback = rules.pop().unwrap();
        let rules = rules.into_iter().map(std::convert::Into::into).collect();
        Self {
            name,
            rules,
            fallback,
        }
    }
}

impl<'a> Workflow<'a> {
    pub fn process_part(&self, part: &Part) -> &str {
        for rule in &self.rules {
            if let Some(dst) = rule.check_part(part) {
                return dst;
            }
        }
        self.fallback
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl std::ops::Index<Metric> for Part {
    type Output = usize;
    fn index(&self, index: Metric) -> &Self::Output {
        match index {
            Metric::X => &self.x,
            Metric::M => &self.m,
            Metric::A => &self.a,
            Metric::S => &self.s,
        }
    }
}

impl Part {
    pub fn score(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    pub fn check(&self, flows: &HashMap<&str, Workflow<'_>>) -> bool {
        let mut flow_key = "in";
        loop {
            flow_key = flows[flow_key].process_part(self);
            if flow_key == "A" {
                break true;
            } else if flow_key == "R" {
                break false;
            }
        }
    }
}

impl FromStr for Part {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        if let Some(caps) = re.captures(s) {
            let x = caps.get(1).unwrap().as_str().parse().unwrap();
            let m = caps.get(2).unwrap().as_str().parse().unwrap();
            let a = caps.get(3).unwrap().as_str().parse().unwrap();
            let s = caps.get(4).unwrap().as_str().parse().unwrap();
            Ok(Self { x, m, a, s })
        } else {
            Err(())
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x={},m={},a={},s={}}}", self.x, self.m, self.a, self.s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status<'a> {
    Pass(Rule<'a>),
    Fail(Rule<'a>),
}

impl<'a> Status<'a> {
    pub fn range(&self) -> (Metric, (usize, usize)) {
        match self {
            Status::Pass(rule) => {
                if rule.op == Comparator::LessThan {
                    (rule.field, (MIN, rule.val - 1))
                } else {
                    (rule.field, (rule.val + 1, MAX))
                }
            }
            Status::Fail(rule) => {
                if rule.op == Comparator::LessThan {
                    (rule.field, (rule.val, MAX))
                } else {
                    (rule.field, (MIN, rule.val))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl std::ops::IndexMut<Metric> for PartRange {
    fn index_mut(&mut self, index: Metric) -> &mut Self::Output {
        match index {
            Metric::X => &mut self.x,
            Metric::M => &mut self.m,
            Metric::A => &mut self.a,
            Metric::S => &mut self.s,
        }
    }
}

impl std::ops::Index<Metric> for PartRange {
    type Output = (usize, usize);
    fn index(&self, index: Metric) -> &Self::Output {
        match index {
            Metric::X => &self.x,
            Metric::M => &self.m,
            Metric::A => &self.a,
            Metric::S => &self.s,
        }
    }
}

impl PartRange {
    pub fn new() -> PartRange {
        PartRange {
            x: (MIN, MAX),
            m: (MIN, MAX),
            a: (MIN, MAX),
            s: (MIN, MAX),
        }
    }
    pub fn pare(&mut self, rules: &[Status<'_>]) {
        for r in rules {
            let (field, (min, max)) = r.range();
            let existing = self[field];
            self[field] = (existing.0.max(min), existing.1.min(max));
        }
    }
    pub fn num_possibilities(&self) -> usize {
        let mut out = 1usize;
        for field in [Metric::X, Metric::M, Metric::A, Metric::S] {
            let rng = self[field];
            if rng.0 > rng.1 {
                return 0;
            }
            out *= rng.1 - rng.0 + 1;
        }
        out
    }
}

impl<'a> From<&Vec<Status<'a>>> for PartRange {
    fn from(value: &Vec<Status<'a>>) -> Self {
        let mut pr = PartRange::new();
        pr.pare(value);
        pr
    }
}

fn backtrack<'a>(
    point: (&str, usize),
    flows: &HashMap<&str, Workflow<'a>>,
    destinations: &HashMap<&str, Vec<&'a str>>,
) -> Vec<Vec<Status<'a>>> {
    let (start_wf, start_idx) = point;
    let mut seed = vec![];
    let flow = &flows[start_wf];
    // If I'm not the fallback, my rule must have passed
    if start_idx < flow.rules.len() {
        seed.push(Status::Pass(flow.rules[start_idx]));
    }
    // So first, all the previous rules in my flow must have FAILED to get here
    for r in 0..start_idx {
        seed.push(Status::Fail(flow.rules[r]));
    }
    // We've reached the root of this flow; the seed is complete
    let seed = seed;
    // Now find everyone that could EVER come back to me
    let mut paths = vec![];
    for (wf, dests) in destinations {
        // Careful! A single rule can have multiple routes to A
        for (d, dst) in dests.iter().enumerate() {
            if dst == &start_wf {
                for path in backtrack((*wf, d), flows, destinations) {
                    let mut extended = seed.clone();
                    extended.extend(path);
                    paths.push(extended);
                }
            }
        }
    }

    if paths.is_empty() {
        vec![seed]
    } else {
        paths
    }
}

fn find_paths<'a>(flows: &HashMap<&str, Workflow<'a>>) -> Vec<Vec<Status<'a>>> {
    let mut paths = vec![];
    // Start at the A's and work our way backwards. They can exist
    // either as the result of a passed rule, OR a fallback.
    let dsts: HashMap<&str, Vec<&str>> = flows
        .values()
        .map(|wf| {
            (
                wf.name,
                wf.rules
                    .iter()
                    .map(|r| r.dst)
                    .chain(std::iter::once(wf.fallback))
                    .collect(),
            )
        })
        .collect();
    for (wf, dests) in &dsts {
        for (d, dst) in dests.iter().enumerate() {
            if dst == &ACCEPTED {
                paths.extend(backtrack((*wf, d), flows, &dsts));
            }
        }
    }
    paths
}

impl Day for Day19 {
    fn task1(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let mut lines = backing.lines();

        let flows: HashMap<&str, Workflow<'_>> = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|wf| {
                let wf: Workflow = wf.into();
                (wf.name, wf)
            })
            .collect();
        let parts: Vec<Part> = lines
            .take_while(|l| !l.is_empty())
            .map(|wf| wf.parse().unwrap())
            .collect();
        let total: usize = parts
            .iter()
            .filter_map(|p| {
                if p.check(&flows) {
                    Some(p.score())
                } else {
                    None
                }
            })
            .sum();
        println!("{total}");
    }
    fn task2(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let flows: HashMap<&str, Workflow<'_>> = backing
            .lines()
            .take_while(|l| !l.is_empty())
            .map(|wf| {
                let wf: Workflow = wf.into();
                (wf.name, wf)
            })
            .collect();

        let total = find_paths(&flows)
            .iter()
            .map(PartRange::from)
            .map(|pr| pr.num_possibilities())
            .sum::<usize>();
        println!("{total}");
    }
}
