use std::fs;

pub fn run() {
    let data = setup();
    let (root, tail) = Node::parse(&data);
    assert!(tail.is_empty());
    println!("Part 1: {}", part1(&root));
    println!("Part 2: {}", part2(&root));
}

fn setup() -> Vec<usize> {
    fs::read_to_string("inputs/08.txt")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part1(root: &Node) -> usize {
    root.sum_metadata()
}

fn part2(root: &Node) -> i32 {
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
        let children: Vec<Node> = (0..num_children)
            .map(|_| {
                let (child, new_tail) = Node::parse(tail);
                tail = new_tail;
                child
            })
            .collect();
        let node = Node {
            children,
            metadata: &tail[..num_metadata],
        };
        (node, &tail[num_metadata..])
    }
    fn sum_my_metadata(&self) -> usize {
        self.metadata.iter().sum()
    }
    fn sum_metadata(&self) -> usize {
        let n: usize = self.children.iter().map(|c| c.sum_metadata()).sum();
        self.sum_my_metadata() + n
    }
    fn value(&self) -> i32 {
        if self.children.is_empty() {
            return self.sum_my_metadata() as i32;
        }
        let num_children = self.children.len();
        let mut memo = vec![-1i32; num_children];
        self.metadata
            .iter()
            .filter(|&m| *m > 0 && *m <= num_children)
            .map(|m| m - 1)
            .map(|idx| {
                if memo[idx] < 0 {
                    memo[idx] = self.children[idx].value();
                }
                memo[idx]
            })
            .sum()
    }
}
