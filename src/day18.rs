use std::fs;

pub fn run() {
    let grid = setup("inputs/18.txt");
    if false {
        print_grid(&grid, 0);
    }
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

fn part1(initial: &[Vec<Cell>]) -> usize {
    let mut grid = simulate(&initial);
    for _t in 1..10 {
        //print_grid(&grid, _t);
        grid = simulate(&grid);
    }
    //print_grid(&grid, 10);
    resource_value(&grid)
}

fn part2(initial: &[Vec<Cell>]) -> usize {
    let mut grid = simulate(&initial);
    for _ in 1..1000 {
        grid = simulate(&grid);
    }
    let mut cycle = vec![resource_value(&grid)];
    for t in 1001..1050 {
        grid = simulate(&grid);
        let v = resource_value(&grid);
        if v == cycle[0] {
            let idx = (1_000_000_000 - t) % cycle.len();
            return cycle[idx];
        }
        cycle.push(v);
    }
    panic!("Cycle not found after 50 iterations.");
}

fn resource_value(grid: &[Vec<Cell>]) -> usize {
    let mut num_wooded = 0;
    let mut num_lumber = 0;
    for row in grid {
        for cell in row {
            match cell {
                Cell::Wooded => {
                    num_wooded += 1;
                }
                Cell::Lumber => {
                    num_lumber += 1;
                }
                _ => {}
            };
        }
    }
    num_wooded * num_lumber
}

fn simulate(grid: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    let mut output = vec![];
    for (r, row) in grid.iter().enumerate() {
        let mut out_row = vec![];
        for (c, cell) in row.iter().enumerate() {
            let (num_wooded, num_lumber) = count_neighbors(r, c, &grid);
            out_row.push(match cell {
                Cell::Open => {
                    if num_wooded >= 3 {
                        Cell::Wooded
                    } else {
                        Cell::Open
                    }
                }
                Cell::Wooded => {
                    if num_lumber >= 3 {
                        Cell::Lumber
                    } else {
                        Cell::Wooded
                    }
                }
                Cell::Lumber => {
                    if num_lumber > 0 && num_wooded > 0 {
                        Cell::Lumber
                    } else {
                        Cell::Open
                    }
                }
            });
        }
        output.push(out_row);
    }
    output
}

fn count_neighbors(r: usize, c: usize, grid: &[Vec<Cell>]) -> (usize, usize) {
    let mut num_wooded = 0;
    let mut num_lumber = 0;
    let mut count_cell = |x: &Cell| match x {
        Cell::Wooded => {
            num_wooded += 1;
        }
        Cell::Lumber => {
            num_lumber += 1;
        }
        _ => {}
    };
    let num_cols = grid[0].len();
    let min_c = if c == 0 { c } else { c - 1 };
    let max_c = if c + 1 == num_cols { c } else { c + 1 };
    if min_c < c {
        count_cell(&grid[r][min_c]);
    }
    if max_c > c {
        count_cell(&grid[r][max_c]);
    }
    if r > 0 {
        for cell in grid[r - 1][min_c..=max_c].iter() {
            count_cell(&cell);
        }
    }
    if r + 1 < grid.len() {
        grid[r + 1][min_c..=max_c].iter().for_each(count_cell);
    }
    (num_wooded, num_lumber)
}

enum Cell {
    Open,
    Wooded,
    Lumber,
}

fn print_grid(grid: &[Vec<Cell>], num_minutes: usize) {
    println!("After {} minutes:", num_minutes);
    for row in grid.iter() {
        let line: String = row
            .iter()
            .map(|c| match c {
                Cell::Open => '.',
                Cell::Wooded => '|',
                Cell::Lumber => '#',
            })
            .collect();
        println!("{}", line);
    }
    println!("");
}

fn setup(path: &str) -> Vec<Vec<Cell>> {
    let data = fs::read_to_string(path).unwrap();
    data.lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    '.' => Cell::Open,
                    '|' => Cell::Wooded,
                    '#' => Cell::Lumber,
                    _ => panic!("Unexpected char: {:?}", c),
                })
                .collect()
        })
        .collect()
}
