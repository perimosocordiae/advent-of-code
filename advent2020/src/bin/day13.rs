use advent2020::*;

fn main() {
    let (t, buses) = parse_input("inputs/13.full");
    println!("part1: {}", part1(t, &buses));
    println!("part2: {}", part2(&buses));
}

#[test]
fn part1_small() {
    let buses = [7, 13, 59, 31, 19];
    assert_eq!(part1(939, &buses), 295);
}

fn part1(t: i64, buses: &[i64]) -> i64 {
    if let Some((w, b)) = buses
        .iter()
        .filter(|&b| *b > 0)
        .map(|b| (b - (t % b), b))
        .min()
    {
        return w * b;
    }
    0
}

#[test]
fn part2_small() {
    let buses = [7, 13, 0, 0, 59, 0, 31, 19];
    assert_eq!(part2(&buses), 1068781);
    assert_eq!(part2(&[17, 0, 13, 19]), 3417);
    assert_eq!(part2(&[67, 7, 59, 61]), 754018);
    assert_eq!(part2(&[67, 0, 7, 59, 61]), 779210);
    assert_eq!(part2(&[67, 7, 0, 59, 61]), 1261476);
    // assert_eq!(part2(&[1789, 37, 47, 1889]), 1202161486);
}

fn part2(buses: &[i64]) -> i64 {
    let mut tmp: Vec<(i64, i64)> = buses
        .iter()
        .enumerate()
        .filter(|(_, &b)| b > 0)
        .map(|(i, &b)| (b, (b - ((i as i64) % b)) % b))
        .collect();
    tmp.sort_unstable();
    println!("tmp = {:?}", tmp);
    let (step, offset) = tmp.pop().unwrap();
    for i in 1.. {
        let t = i * step + offset;
        if tmp.iter().all(|(b, off)| t % b == *off) {
            return t;
        }
    }
    unreachable!("Exited infinite loop");
}

fn parse_input(path: &str) -> (i64, Vec<i64>) {
    let data = read_string(path);
    let lines: Vec<&str> = data.lines().collect();
    let t: i64 = lines[0].parse().unwrap();
    let mut buses = vec![];
    for part in lines[1].split(',') {
        buses.push(if part == "x" {
            0
        } else {
            part.parse().unwrap()
        })
    }
    (t, buses)
}
