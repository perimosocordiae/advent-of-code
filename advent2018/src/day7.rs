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
            if has_no_incoming(&g, i) {
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

fn part2(graph: &DiGraphMap<char, ()>) -> i32 {
    let mut elapsed: i32 = 0;
    let mut g = graph.clone();
    let mut workers = [Worker { node: '0', time: 0 }; 5];
    while g.node_count() > 0 {
        let ready: Vec<usize> = workers
            .iter()
            .enumerate()
            .filter(|(_, w)| w.time <= elapsed)
            .map(|(idx, _)| idx)
            .collect();
        let mut did_work = false;
        for idx in ready {
            if workers[idx].node != '0' {
                g.remove_node(workers[idx].node);
            }
            let mut nodes: Vec<char> = g.node_identifiers().collect();
            nodes.sort_unstable();
            for i in nodes.into_iter() {
                if workers.iter().any(|w| w.node == i) {
                    continue;
                }
                if has_no_incoming(&g, i) {
                    workers[idx] = Worker {
                        node: i,
                        time: elapsed + cost(i),
                    };
                    did_work = true;
                    break;
                }
            }
        }
        if !did_work {
            elapsed += 1;
        }
    }
    elapsed - 1
}

#[derive(Debug, Clone, Copy)]
struct Worker {
    node: char,
    time: i32,
}

fn has_no_incoming(g: &DiGraphMap<char, ()>, node: char) -> bool {
    g.neighbors_directed(node, Direction::Incoming)
        .next()
        .is_none()
}

fn cost(node: char) -> i32 {
    let c = node.to_ascii_uppercase() as u8 - 64;
    i32::from(c) + 60
}
