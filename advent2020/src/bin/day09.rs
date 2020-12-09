use advent2020::*;
use itertools::Itertools;

fn main() {
    let nums = read_integers("inputs/09.full");
    let target = part1(25, &nums);
    println!("Part 1: {}", target);
    println!("Part 2: {}", part2(target, &nums));
}

#[test]
fn test_part1() {
    let nums = read_integers("inputs/09.test");
    assert_eq!(part1(5, &nums), 127);
}

fn part1(preamble: usize, nums: &[i64]) -> i64 {
    for (start, num) in nums[preamble..].iter().enumerate() {
        let stop = start + preamble;
        if !can_sum(*num, &nums[start..stop]) {
            return *num;
        }
    }
    -1
}

fn can_sum(target: i64, nums: &[i64]) -> bool {
    for pair in nums.iter().combinations(2) {
        if pair[0] == pair[1] {
            continue;
        }
        if pair[0] + pair[1] == target {
            return true;
        }
    }
    false
}

#[test]
fn test_part2() {
    let nums = read_integers("inputs/09.test");
    assert_eq!(part2(127, &nums), 62);
}

fn part2(target: i64, nums: &[i64]) -> i64 {
    for i in 0..(nums.len() - 1) {
        let mut sum: i64 = 0;
        for (j, x) in nums[i..].iter().enumerate() {
            sum += x;
            if sum == target {
                let seq = &nums[i..(i + j)];
                return seq.iter().min().unwrap() + seq.iter().max().unwrap();
            } else if sum > target {
                break;
            }
        }
    }
    -1
}
