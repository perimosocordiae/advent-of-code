use advent2020::*;
use std::collections::HashMap;

fn main() {
    let nums = read_integers("inputs/10.full");
    println!("Part 1: {}", part1(&nums));
    println!("Part 2: {}", part2(&nums));
}

#[test]
fn test_part1_small() {
    let nums = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    assert_eq!(part1(&nums), 7 * 5);
}

#[test]
fn test_part1() {
    let nums = read_integers("inputs/10.test");
    assert_eq!(part1(&nums), 22 * 10);
}

fn part1(nums: &[i64]) -> usize {
    let adapters = prep_adapters(&nums);
    // bincount(diff(adapters))
    let mut counts = [0; 4];
    for diff in adapters.windows(2).map(|w| w[1] - w[0]) {
        counts[diff] += 1;
    }
    counts[1] * counts[3]
}

// sort and add implicit 0 and max + 3 entries, converting to usize.
fn prep_adapters(nums: &[i64]) -> Vec<usize> {
    let mut adapters = vec![0];
    for x in nums {
        adapters.push(*x as usize);
    }
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);
    adapters
}

#[test]
fn test_part2_small() {
    let nums = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    assert_eq!(part2(&nums), 8);
}

#[test]
fn test_part2() {
    let nums = read_integers("inputs/10.test");
    assert_eq!(part2(&nums), 19208);
}

fn part2(nums: &[i64]) -> usize {
    let adapters = prep_adapters(&nums);
    let mut memo = HashMap::new();
    count_paths(&adapters, 0, &mut memo)
}

fn count_paths(
    full_adapters: &[usize],
    idx: usize,
    mut memo: &mut HashMap<usize, usize>,
) -> usize {
    let adapters = &full_adapters[idx..];
    if adapters.len() <= 2 {
        return 1;
    }
    if let Some(count) = memo.get(&idx) {
        return *count;
    }
    let limit = adapters[0] + 3;
    let mut res = 0;
    for (i, x) in adapters.iter().enumerate().skip(1) {
        if *x <= limit {
            res += count_paths(full_adapters, idx + i, &mut memo);
        } else {
            break;
        }
    }
    memo.insert(idx, res);
    res
}
