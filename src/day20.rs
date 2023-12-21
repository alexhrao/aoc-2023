use std::{collections::HashMap, fs};

use regex::Regex;

use super::Day;

pub struct Day20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SignalSource<'a>(&'a str, Signal);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Off,
    On,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop(Status),
    Conjunction(HashMap<&'a str, Signal>),
}

impl<'a> ModuleType<'a> {
    pub fn from_str(value: &'a str) -> ModuleType<'a> {
        match value.chars().next().unwrap() {
            'b' => ModuleType::Broadcaster,
            '%' => ModuleType::FlipFlop(Status::Off),
            '&' => ModuleType::Conjunction(HashMap::new()),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Module<'a> {
    name: &'a str,
    module_type: ModuleType<'a>,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

impl<'a> Module<'a> {
    pub fn new(name: &'a str) -> Module<'a> {
        if name == "broadcaster" {
            Module {
                name,
                module_type: ModuleType::Broadcaster,
                inputs: vec![],
                outputs: vec![],
            }
        } else {
            let module_type = ModuleType::from_str(name);
            let name = &name[1..];
            Module {
                name,
                module_type,
                inputs: vec![],
                outputs: vec![],
            }
        }
    }

    pub fn kick(&mut self, input: SignalSource<'a>) -> Option<Signal> {
        let SignalSource(src, sig) = input;
        match &mut self.module_type {
            ModuleType::Broadcaster => Some(sig),
            ModuleType::Conjunction(memory) => {
                // First set. then check
                memory.insert(src, sig);
                Some(if memory.values().all(|s| s == &Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                })
            }
            ModuleType::FlipFlop(status) => match sig {
                Signal::High => None,
                Signal::Low => Some(if status == &Status::Off {
                    *status = Status::On;
                    Signal::High
                } else {
                    *status = Status::Off;
                    Signal::Low
                }),
            },
        }
    }
}

fn parse_modules(backing: &str) -> HashMap<&str, Module<'_>> {
    let re = Regex::new(r"([%&]?\w+) -> (.*)").unwrap();
    let mods: Vec<_> = backing
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let mut m = Module::new(caps.get(1).unwrap().as_str());
            m.outputs = caps.get(2).unwrap().as_str().split(", ").collect();
            m
        })
        .collect();
    let mut maps = HashMap::new();
    for m in &mods {
        for out in &m.outputs {
            maps.entry(*out).or_insert(vec![]).push(m.name);
        }
    }
    let mut mods: HashMap<_, _> = mods.into_iter().map(|m| (m.name, m)).collect();
    for (n, ins) in maps {
        if let Some(m) = mods.get_mut(n) {
            m.inputs = ins;
            if let ModuleType::Conjunction(sigs) = &mut m.module_type {
                *sigs = m.inputs.iter().map(|i| (*i, Signal::Low)).collect();
            }
        }
    }
    mods
}

fn button_press(mods: &mut HashMap<&str, Module<'_>>) -> (usize, usize) {
    // Button => Broadcaster => Everyone else
    let mut queues = HashMap::new();
    let mut lows = 0;
    let mut highs = 0;
    // Kick it off
    queues.insert("broadcaster", vec![SignalSource("button", Signal::Low)]);
    while queues.values().any(|q| !q.is_empty()) {
        // loop over the queues
        let mut next = HashMap::new();
        for (module, queue) in queues {
            // This is the queue for module
            for sig in queue {
                // Add it as we process it
                if sig.1 == Signal::Low {
                    lows += 1;
                } else {
                    highs += 1;
                }
                if let Some(m) = mods.get_mut(module) {
                    // This module may or may not exist, so only kick it if
                    // we have someone to kick!
                    if let Some(s) = m.kick(sig) {
                        for out in &m.outputs {
                            next.entry(*out)
                                .or_insert(vec![])
                                .push(SignalSource(module, s));
                        }
                    }
                }
            }
        }
        queues = next;
    }
    (lows, highs)
}

impl Day for Day20 {
    fn task1(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let mut mods = parse_modules(&backing);

        let mut lows = 0;
        let mut highs = 0;

        for _ in 0..1000 {
            let (l, h) = button_press(&mut mods);
            lows += l;
            highs += h;
        }

        println!("{}", lows * highs);
    }
    fn task2(&self, _file: &std::path::Path) {}
}
