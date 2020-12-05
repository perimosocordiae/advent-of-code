use advent2020::*;

fn main() {
    let data = read_string("inputs/05.full");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[test]
fn part1_small() {
    assert_eq!(part1("FBFBBFFRLR"), 357);
    assert_eq!(part1("BFFFBBFRRR"), 567);
    assert_eq!(part1("FFFBBBFRRR"), 119);
    assert_eq!(part1("BBFFBBFRLL"), 820);
    assert_eq!(part1("BFFFBBFRRR\nBBFFBBFRLL\nFFFBBBFRRR"), 820);
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|s: &str| decode_seat(&s))
        .max()
        .unwrap_or(0)
}

fn part2(input: &str) -> i32 {
    let mut seats: Vec<i32> =
        input.lines().map(|s: &str| decode_seat(&s)).collect();
    seats.sort_unstable();
    if let Some(w) = seats.windows(2).find(|&w| w[0] + 1 != w[1]) {
        return w[0] + 1;
    }
    0
}

fn decode_seat(code: &str) -> i32 {
    let mut low_row = 0;
    let mut high_row = 127;
    let mut low_col = 0;
    let mut high_col = 7;
    for ch in code.chars() {
        let mid_row = (high_row + low_row) / 2;
        let mid_col = (high_col + low_col) / 2;
        match ch {
            'F' => high_row = mid_row,
            'B' => low_row = mid_row + 1,
            'L' => high_col = mid_col,
            'R' => low_col = mid_col + 1,
            _ => {}
        }
    }
    low_row * 8 + low_col
}
