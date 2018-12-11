use ndarray::prelude::*;

pub fn run() {
    let grid = setup();
    let (x, y) = part1(&grid);
    println!("Part 1: {},{}", x, y);
    let (x, y, conv_size) = part2(&grid);
    println!("Part 2: {},{},{}", x, y, conv_size);
}

fn setup() -> Array2<i32> {
    let serial_number = 1723;
    let mut grid = Array::from_elem((301, 301), -9999);
    for y in 1..=300 {
        for x in 1..=300 {
            grid[[y, x]] = power_level(x as i32, y as i32, serial_number);
        }
    }
    grid
}

fn part1(grid: &Array2<i32>) -> (usize, usize) {
    let nr = grid.len_of(Axis(0));
    let nc = grid.len_of(Axis(1));
    let (y, x) = grid
        .slice(s![1..(nr - 1), 1..(nc - 1)])
        .indexed_iter()
        .max_by_key(|((yy, xx), _)| grid.slice(s![*yy..(yy + 3), *xx..(xx + 3)]).sum())
        .unwrap()
        .0;
    (x, y)
}

fn part2(grid: &Array2<i32>) -> (usize, usize, usize) {
    let nr = grid.len_of(Axis(0));
    let nc = grid.len_of(Axis(1));
    let tmp = grid.indexed_iter().max_by_key(|(_, v)| *v).unwrap();
    let mut best_yx = tmp.0;
    let mut best_val = *tmp.1;
    let mut best_conv_size = 1;
    for conv_size in 2..=300 {
        let slice = grid.slice(s![..(nr - conv_size), ..(nc - conv_size)]);
        let (val, yx) = slice
            .indexed_iter()
            .map(|((yy, xx), _)| {
                let val = grid
                    .slice(s![yy..(yy + conv_size), xx..(xx + conv_size)])
                    .sum();
                (val, (yy, xx))
            })
            .max()
            .unwrap()
            .clone();
        if val > best_val {
            best_val = val;
            best_yx = yx;
            best_conv_size = conv_size;
        }
    }
    (best_yx.1, best_yx.0, best_conv_size)
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y + serial_number;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;
    power_level - 5
}
