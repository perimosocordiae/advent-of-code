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
    let mut marbles = vec![0usize];
    let mut scores = vec![0usize; num_players];
    let mut next_marble: usize = 1;
    let mut curr_idx: usize = 0;
    loop {
        for player in 0..num_players {
            if next_marble % 23 == 0 {
                curr_idx = (curr_idx + marbles.len() - 7) % marbles.len();
                scores[player] += next_marble + marbles.remove(curr_idx);
            } else {
                curr_idx = (curr_idx + 1) % marbles.len() + 1;
                if curr_idx == marbles.len() {
                    marbles.push(next_marble);
                } else {
                    marbles.insert(curr_idx, next_marble);
                }
            }
            next_marble += 1;
            if next_marble == last_marble {
                return scores.into_iter().max().unwrap();
            }
        }
    }
}

fn part2(num_players: usize, last_marble: usize) -> usize {
    part1(num_players, last_marble * 100)
}
