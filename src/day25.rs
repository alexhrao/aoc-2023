use aoc_runner_derive::aoc;
use petgraph::prelude::*;
use regex::Regex;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use std::collections::HashMap;

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

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let modules = parse(input);
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
    nodes.len() * (graph.node_count() - nodes.len())
}
