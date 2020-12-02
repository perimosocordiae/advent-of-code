use advent2020::*;

fn main() {
    foo::demo();
    let nums = read_integers("inputs/01.full");
    println!("Part 1: {}", part1(&nums));
    println!("Part 2: {}", part2(&nums));
}

#[test]
fn part1_small() {
    let nums = read_integers("inputs/01.test");
    assert_eq!(part1(&nums), 514579);
}

#[test]
fn part2_small() {
    let nums = read_integers("inputs/01.test");
    assert_eq!(part2(&nums), 241861950);
    assert_eq!(part2(&[2018, 1, 1]), 2018);
}

fn part1(nums: &[i64]) -> i64 {
    for (i, x) in nums.iter().enumerate() {
        let target = 2020 - x;
        if let Some(y) = nums.iter().skip(i + 1).find(|&&y| y == target) {
            return x * y;
        }
    }
    -1
}

fn part2(nums: &[i64]) -> i64 {
    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().skip(i + 1).enumerate() {
            let target = 2020 - x - y;
            if let Some(z) = nums.iter().skip(i + j + 2).find(|&&z| z == target)
            {
                return x * y * z;
            }
        }
    }
    -1
}
