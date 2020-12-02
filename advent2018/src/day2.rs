use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn setup() -> Vec<String> {
    let f = File::open("inputs/02.txt").unwrap();
    BufReader::new(&f).lines().map(|l| l.unwrap()).collect()
}

fn part1(data: &[String]) -> i32 {
    let mut num_doubles: i32 = 0;
    let mut num_triples: i32 = 0;
    for id in data {
        let mut counts: HashMap<u8, i32> = HashMap::new();
        for c in id.as_bytes() {
            let count = counts.entry(*c).or_insert(0);
            *count += 1;
        }
        let uniq_counts: HashSet<i32> = counts.values().cloned().collect();
        for count in uniq_counts {
            if count == 2 {
                num_doubles += 1;
            } else if count == 3 {
                num_triples += 1;
            }
        }
    }
    num_doubles * num_triples
}

fn part2(data: &[String]) -> String {
    let n = data.len();
    let m = data[0].len();
    for (idx, a) in data[..n - 1].iter().enumerate() {
        for b in &data[idx + 1..n] {
            let shared_chars = a
                .as_bytes()
                .iter()
                .zip(b.as_bytes())
                .filter(|(x, y)| x == y)
                .map(|(x, _)| *x)
                .collect::<Vec<u8>>();
            if shared_chars.len() == m - 1 {
                return String::from_utf8(shared_chars).unwrap();
            }
        }
    }
    String::from("No match found!")
}
