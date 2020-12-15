use std::collections::HashMap;

fn main() {
    let numbers = [1, 20, 8, 12, 0, 14];
    println!("part1: {}", say_numbers(&numbers, 2020));
    println!("part2: {}", say_numbers(&numbers, 30_000_000));
}

#[test]
fn test_part1() {
    assert_eq!(say_numbers(&[0, 3, 6], 10), 0);
    assert_eq!(say_numbers(&[0, 3, 6], 2020), 436);
}

// This is too slow when run in non-release mode.
// #[test]
// fn test_part2() {
//     assert_eq!(say_numbers(&[0, 3, 6], 30_000_000), 175594);
//     assert_eq!(say_numbers(&[1, 3, 2], 30_000_000), 2578);
// }

fn say_numbers(starting_nums: &[usize], last_turn: usize) -> usize {
    let mut memory = HashMap::<usize, usize>::new();
    for (i, x) in starting_nums[..(starting_nums.len() - 1)]
        .iter()
        .enumerate()
    {
        memory.insert(*x, i + 1);
    }
    let mut last_number = *starting_nums.last().unwrap();
    for turn in starting_nums.len()..last_turn {
        let last_seen = memory.entry(last_number).or_insert_with(|| turn);
        last_number = turn - *last_seen;
        *last_seen = turn;
    }
    last_number
}
