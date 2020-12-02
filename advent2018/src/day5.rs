use std::fs;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn setup() -> Vec<PolymerUnit> {
    // let data: Vec<u8> = "dabAcCaCBAcCcaDA".as_bytes().to_vec();
    let data = fs::read("inputs/05.txt").expect("Unable to read input.");
    let polymer: Vec<PolymerUnit> = data
        .into_iter()
        .filter(|&c| c >= 65)
        .map(PolymerUnit::new)
        .collect();
    react_fully(&polymer)
}

fn part1(data: &[PolymerUnit]) -> usize {
    data.len()
}

fn part2(data: &[PolymerUnit]) -> usize {
    (0..26)
        .map(|id| remove_id(&data, id))
        .map(|poly| react_fully(&poly).len())
        .min()
        .unwrap()
}

fn remove_id(data: &[PolymerUnit], id: u8) -> Vec<PolymerUnit> {
    data.iter().filter(|&u| u.id != id).cloned().collect()
}

fn react_fully(data: &[PolymerUnit]) -> Vec<PolymerUnit> {
    let reduced = react_one_pass(data);
    if reduced.len() == data.len() {
        reduced
    } else {
        react_fully(&reduced)
    }
}

fn react_one_pass(data: &[PolymerUnit]) -> Vec<PolymerUnit> {
    let mut kept = Vec::new();
    let n = data.len();
    let mut i: usize = 0;
    let mut j: usize = 1;
    while i < n && j < n {
        if data[i].reacts_with(data[j]) {
            i += 2;
            j += 2;
        } else {
            kept.push(data[i]);
            i += 1;
            j += 1;
        }
    }
    if i < n {
        kept.push(data[i]);
    }
    kept
}

#[derive(Debug, Clone, Copy)]
struct PolymerUnit {
    id: u8,
    polarity: bool,
}

impl PolymerUnit {
    fn new(c: u8) -> PolymerUnit {
        let polarity = c < 97;
        PolymerUnit {
            id: if polarity { c - 65 } else { c - 97 },
            polarity,
        }
    }
    fn reacts_with(self, other: PolymerUnit) -> bool {
        self.id == other.id && self.polarity != other.polarity
    }
}
