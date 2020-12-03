use advent2020::*;

fn main() {
    let input = read_string("inputs/03.full");
    let map: Vec<&str> = input.lines().collect();
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

#[test]
fn tree_counting() {
    let input = read_string("inputs/03.test");
    let map: Vec<&str> = input.lines().collect();
    assert_eq!(count_trees(&map, 1, 1), 2);
    assert_eq!(count_trees(&map, 1, 3), 7);
    assert_eq!(count_trees(&map, 1, 5), 3);
    assert_eq!(count_trees(&map, 1, 7), 4);
    assert_eq!(count_trees(&map, 2, 1), 2);
}

fn part1(map: &[&str]) -> usize {
    count_trees(&map, 1, 3)
}

fn part2(map: &[&str]) -> usize {
    count_trees(&map, 1, 1)
        * count_trees(&map, 1, 3)
        * count_trees(&map, 1, 5)
        * count_trees(&map, 1, 7)
        * count_trees(&map, 2, 1)
}

fn count_trees(map: &[&str], d_row: usize, d_col: usize) -> usize {
    let num_cols = map.first().unwrap().len();
    let mut num_trees = 0;
    let mut col = 0;
    for map_row in map.iter().step_by(d_row) {
        if map_row.chars().nth(col).unwrap() == '#' {
            num_trees += 1;
        }
        col += d_col;
        col %= num_cols;
    }
    num_trees
}
