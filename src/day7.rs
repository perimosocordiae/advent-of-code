use petgraph::prelude::*;
use petgraph::visit::IntoNodeIdentifiers;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn setup() -> DiGraphMap<char, ()> {
    let f = File::open("inputs/07.txt").unwrap();
    let mut g = DiGraphMap::new();
    BufReader::new(&f)
        .lines()
        .map(|l| {
            let parts: Vec<char> = l.unwrap().split_whitespace().map(first_char).collect();
            (parts[1], parts[7])
        })
        .for_each(|(a, b)| {
            g.add_edge(a, b, ());
        });
    g
}

fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

fn part1(graph: &DiGraphMap<char, ()>) -> String {
    let mut nodes: Vec<char> = graph.node_identifiers().collect();
    nodes.sort_unstable();
    let mut g = graph.clone();
    let mut order = Vec::new();
    while !nodes.is_empty() {
        for i in nodes.into_iter() {
            if g.neighbors_directed(i, Direction::Incoming)
                .next()
                .is_none()
            {
                order.push(i);
                g.remove_node(i);
                break;
            }
        }
        nodes = g.node_identifiers().collect();
        nodes.sort_unstable();
    }
    order.into_iter().collect()
}

fn part2(data: &DiGraphMap<char, ()>) -> i32 {
    let mut elapsed: i32 = 0;
    elapsed
}

fn cost(node: char) -> i32 {
    let c = node.to_ascii_uppercase() as u8 - 97;
    i32::from(c)
}
