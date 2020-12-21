use advent2020::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[macro_use]
extern crate scan_fmt;

fn main() {
    let tiles = parse_tiles("inputs/20.full");
    println!("part1: {}", part1(&tiles));
}

#[test]
fn part1_small() {
    let tiles = parse_tiles("inputs/20.test");
    assert_eq!(part1(&tiles), 20899048083289);
}

fn part1(tiles: &HashMap<usize, Tile>) -> usize {
    let mut sketches = HashMap::<String, Vec<usize>>::new();
    for (tile_id, tile) in tiles {
        for side in tile.edges.iter() {
            sketches
                .entry(side.to_string())
                .or_insert_with(Vec::new)
                .push(*tile_id);
            sketches
                .entry(side.chars().rev().collect())
                .or_insert_with(Vec::new)
                .push(*tile_id);
        }
    }
    let mut graph = HashMap::<usize, HashSet<usize>>::new();
    for bucket in sketches.values() {
        for pair in bucket.iter().combinations(2) {
            let a = pair[0];
            let b = pair[1];
            graph.entry(*a).or_insert_with(HashSet::new).insert(*b);
            graph.entry(*b).or_insert_with(HashSet::new).insert(*a);
        }
    }
    graph
        .iter()
        .filter(|(_, neighbors)| neighbors.len() == 2)
        .map(|(tile_id, _)| tile_id)
        .product()
}

#[derive(Debug)]
struct Tile {
    // order is clockwise: top, right, bottom, left
    edges: Vec<String>,
    // TODO: store the image (non-border parts of the tile) for part 2.
}

fn parse_tiles(path: &str) -> HashMap<usize, Tile> {
    let data = read_string(path);
    let mut out = HashMap::new();
    for chunk in data.split("\n\n") {
        let tile_id = scan_fmt!(&chunk, "Tile {d}:", usize).unwrap();
        let tile_lines: Vec<&str> = chunk.lines().skip(1).collect();
        let n = tile_lines.len();
        let top = tile_lines[0].to_string();
        let right: String = (0..n)
            .map(|i| tile_lines[i].chars().last().unwrap())
            .collect();
        let left: String = (0..n)
            .map(|i| tile_lines[i].chars().next().unwrap())
            .collect();
        let bottom = tile_lines[n - 1].to_string();
        out.insert(
            tile_id,
            Tile {
                edges: vec![top, right, bottom, left],
            },
        );
    }
    out
}
