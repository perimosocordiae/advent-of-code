use std::fs;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn setup() -> Vec<usize> {
    fs::read_to_string("inputs/08.txt")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part1(data: &[usize]) -> usize {
    let (root, tail) = Node::parse(&data);
    assert!(tail.is_empty());
    root.sum_metadata()
}

fn part2(data: &[usize]) -> usize {
    let (root, tail) = Node::parse(&data);
    assert!(tail.is_empty());
    root.value()
}

#[derive(Debug)]
struct Node<'a> {
    children: Vec<Node<'a>>,
    metadata: &'a [usize],
}

impl<'a> Node<'a> {
    fn parse(data: &[usize]) -> (Node, &[usize]) {
        let num_children = data[0];
        let num_metadata = data[1];
        let mut tail = &data[2..];
        let mut children: Vec<Node> = vec![];
        for _ in 0..num_children {
            let (child, new_tail) = Node::parse(tail);
            children.push(child);
            tail = new_tail;
        }
        let node = Node {
            children,
            metadata: &tail[..num_metadata],
        };
        (node, &tail[num_metadata..])
    }
    fn sum_metadata(&self) -> usize {
        let m: usize = self.metadata.iter().sum();
        let n: usize = self.children.iter().map(|c| c.sum_metadata()).sum();
        m + n
    }
    fn value(&self) -> usize {
        if self.children.is_empty() {
            return self.metadata.iter().sum();
        }
        let num_children = self.children.len();
        let mut memo = vec![-1i32; num_children];
        self.metadata
            .iter()
            .filter(|&m| *m > 0 && *m <= num_children)
            .map(|m| m - 1)
            .map(|idx| {
                if memo[idx] < 0 {
                    memo[idx] = self.children[idx].value() as i32;
                }
                memo[idx] as usize
            })
            .sum()
    }
}
