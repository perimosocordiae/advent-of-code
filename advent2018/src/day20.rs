use std::fs;
use std::rc::Rc;

pub fn run() {
    let regex = setup("inputs/20-small.txt");
    println!("Part 1: {}", part1(&regex));
}

fn part1(_regex: &RegexTree) -> usize {
    0
}

struct RegexTree {
    parts: Vec<RegexPiece>,
    parent: Option<Rc<RegexTree>>,
}

enum RegexPiece {
    Leaf(String),
    Node(Rc<RegexTree>),
}

impl RegexTree {
    /*
        fn add_literal(&mut self, c: char) {
            if let Some(RegexPiece::Leaf(s)) = self.parts.last_mut() {
                s.push(c);
            } else {
                self.parts.push(RegexPiece::Leaf(c.to_string()));
            }
        }
        fn start_group(&mut self) -> &mut RegexTree {
            let mut child = RegexTree { parts: vec![], parent: Some(Rc::new(*self)) };
            let mut tmp = Rc::new(child);
            self.parts.push(RegexPiece::Node(tmp.clone()));
            Rc::get_mut(&mut tmp).unwrap()
        }
        fn end_group(&mut self) -> &mut RegexTree {
            &mut *self.parent.unwrap()
        }
        fn next_case(&mut self) {
            self.parts.push(RegexPiece::Leaf(String::new()));
        }
    */
}

fn setup(path: &str) -> RegexTree {
    let data = fs::read_to_string(path).unwrap();
    parse_pattern(&data[1..data.len() - 1]) // trim ^ and $
}

fn parse_pattern(pattern: &str) -> RegexTree {
    let mut root = RegexTree {
        parts: vec![],
        parent: None,
    };
    /*
    let mut node = &mut root;
    for c in pattern.chars() {
        match c {
            'N' | 'S' | 'E' | 'W' => {node.add_literal(c)},
            '(' => {node = node.start_group()},
            ')' => {node = node.end_group()},
            '|' => {node.next_case()},
            _ => panic!("Unexpected char in pattern: {}", c)
        };
    }
    */
    root
}
