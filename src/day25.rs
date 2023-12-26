use std::{collections::HashMap, fs};

use super::Day;
use petgraph::prelude::*;
use regex::Regex;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

pub struct Day25;

fn parse(backing: &str) -> HashMap<&str, Vec<&str>> {
    let re = Regex::new(r"([^:]*):(.*)").unwrap();
    backing
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let nodes = caps.get(2).unwrap().as_str().split_whitespace().collect();
            (caps.get(1).unwrap().as_str(), nodes)
        })
        .collect()
}

impl Day for Day25 {
    fn task1(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let modules = parse(&backing);
        let mut graph: Graph<&str, (), Undirected, u32> = Graph::default();
        let mut nodes = HashMap::new();
        for &name in modules
            .keys()
            .chain(modules.values().flat_map(|m| m.iter()))
        {
            nodes.entry(name).or_insert_with(|| graph.add_node(name));
        }
        for (&src, dsts) in &modules {
            for &d in dsts {
                graph.add_edge(nodes[src], nodes[d], ());
            }
        }

        let (len, nodes) = stoer_wagner_min_cut(&graph, |_| Result::<usize, ()>::Ok(1))
            .unwrap()
            .unwrap();
        assert_eq!(len, 3);
        println!("{}", nodes.len() * (graph.node_count() - nodes.len()));
    }
    fn task2(&self, _file: &std::path::Path) {}
}
