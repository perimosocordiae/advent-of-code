use std::collections::VecDeque;
use std::fs;

pub fn run() {
    let (num_players, last_marble) = setup();
    println!("Part 1: {}", part1(num_players, last_marble));
    println!("Part 2: {}", part2(num_players, last_marble));
}

fn setup() -> (usize, usize) {
    let line = fs::read_to_string("inputs/09.txt").unwrap();
    let parts: Vec<&str> = line.split_whitespace().collect();
    (parts[0].parse().unwrap(), parts[6].parse().unwrap())
}

fn part1(num_players: usize, last_marble: usize) -> usize {
    let mut marbles = VecDeque::new();
    marbles.push_front(0);
    let mut scores = vec![0usize; num_players];
    let mut next_marble: usize = 1;
    loop {
        for player in 0..num_players {
            if next_marble % 23 == 0 {
                rotate_right(&mut marbles, 7);
                scores[player] += next_marble + marbles.pop_back().unwrap();
                rotate_left(&mut marbles, 1);
            } else {
                rotate_left(&mut marbles, 1);
                marbles.push_back(next_marble);
            }
            next_marble += 1;
            if next_marble == last_marble {
                return scores.into_iter().max().unwrap();
            }
        }
    }
}

fn rotate_right(deque: &mut VecDeque<usize>, k: usize) {
    for _ in 0..k {
        let tmp = deque.pop_back().unwrap();
        deque.push_front(tmp);
    }
}

fn rotate_left(deque: &mut VecDeque<usize>, k: usize) {
    for _ in 0..k {
        let tmp = deque.pop_front().unwrap();
        deque.push_back(tmp);
    }
}

fn part2(num_players: usize, last_marble: usize) -> usize {
    part1(num_players, last_marble * 100)
}
