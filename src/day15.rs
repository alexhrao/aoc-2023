use std::{fs, str::FromStr};

use super::Day;

pub struct Day15;

fn digest(s: &str) -> usize {
    let mut working = 0;
    for c in s.as_bytes() {
        let tmp = *c as usize;
        working += tmp;
        working *= 17;
        working %= 256;
    }
    working
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    Set(usize),
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    id: String,
    op: Operation,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('-') {
            Ok(Self {
                id: String::from(&s[0..s.len() - 1]),
                op: Operation::Remove,
            })
        } else {
            let mut split = s.split('=');
            let id = String::from(split.next().unwrap());
            let op = Operation::Set(split.next().unwrap().parse().unwrap());
            Ok(Self { id, op })
        }
    }
}

fn total_bucket(bucket: &[(String, usize)], b: usize) -> usize {
    bucket
        .iter()
        .enumerate()
        .map(|(i, inst)| (b + 1) * (i + 1) * inst.1)
        .sum::<usize>()
}

impl Day for Day15 {
    fn task1(&self, file: &std::path::Path) {
        let total = fs::read_to_string(file)
            .unwrap()
            .replace('\n', "")
            .split(',')
            .map(digest)
            .sum::<usize>();
        println!("{}", total);
    }
    fn task2(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap().replace('\n', "");
        let steps: Vec<Instruction> = backing.split(',').map(|s| s.parse().unwrap()).collect();
        // println!("{:?}", steps);

        let mut map = vec![vec![]; 256];
        for s in steps {
            let bucket = &mut map[digest(&s.id)];
            let idx = bucket.iter().position(|i: &(String, usize)| i.0 == s.id);

            match s.op {
                Operation::Remove => {
                    if let Some(idx) = idx {
                        bucket.remove(idx);
                    }
                }
                Operation::Set(len) => {
                    if let Some(idx) = idx {
                        bucket[idx] = (s.id, len);
                    } else {
                        bucket.push((s.id, len));
                    }
                }
            };
        }
        let total = map
            .iter()
            .enumerate()
            .map(|(b, bucket)| total_bucket(bucket, b))
            .sum::<usize>();
        println!("{:?}", total);
    }
}
