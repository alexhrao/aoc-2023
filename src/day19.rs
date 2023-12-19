use std::{collections::HashMap, fs, str::FromStr};

use super::Day;
use regex::Regex;

pub struct Day19;

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
    cmp: Comparator,
    val: usize,
    dst: &'a str,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        let re = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
        if let Some(caps) = re.captures(s) {
            let field = caps.get(1).unwrap().as_str().parse().unwrap();
            let cmp = caps.get(2).unwrap().as_str().parse().unwrap();
            let val = caps.get(3).unwrap().as_str().parse().unwrap();
            let dst = caps.get(4).unwrap().as_str();
            Self {
                field,
                cmp,
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
        let result = match self.cmp {
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
        let rules = rules.into_iter().map(|r| r.into()).collect();
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
        println!("{}", total);
    }
    fn task2(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let _flows: HashMap<&str, Workflow<'_>> = backing
            .lines()
            .take_while(|l| !l.is_empty())
            .map(|wf| {
                let wf: Workflow = wf.into();
                (wf.name, wf)
            })
            .collect();
    }
}
