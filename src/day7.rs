use petgraph::prelude::*;
use petgraph::visit::{Dfs, IntoNodeIdentifiers, VisitMap, Visitable};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    // println!("Part 2: {}", part2(&data));
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

fn part1(g: &DiGraphMap<char, ()>) -> String {
    // let mut order = Vec::<char>::new();
    // while let Some(nx) = dfs.next(&data) {
    //     order.push(nx);
    //     dfs.stack.sort_unstable_by(|a, b| b.cmp(a));
    //     println!("{:?} {:?}", order, dfs.stack);
    // }
    // order.into_iter().collect()
    let mut dfs = Dfs::empty(&g);
    let mut finished = g.visit_map();
    let mut finish_stack = Vec::new();
    let mut nodes: Vec<char> = g.node_identifiers().collect();
    nodes.sort_unstable_by(|a, b| b.cmp(a));
    for i in nodes {
        if dfs.discovered.is_visited(&i) {
            continue;
        }
        dfs.stack.push(i);
        while let Some(&nx) = dfs.stack.last() {
            if dfs.discovered.visit(nx) {
                // First time visiting `nx`: Push neighbors, don't pop `nx`
                for succ in g.neighbors(nx) {
                    if !dfs.discovered.is_visited(&succ) {
                        dfs.stack.push(succ);
                    }
                }
            } else {
                dfs.stack.pop();
                if finished.visit(nx) {
                    // Second time: All reachable nodes must have been finished
                    finish_stack.push(nx);
                }
            }
        }
    }
    finish_stack.into_iter().rev().collect()
}

// fn part2(data: &DiGraphMap<(), ()>) -> i32 {
//     0
// }
