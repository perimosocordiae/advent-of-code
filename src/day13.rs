use std::fs;

pub fn run() {
    let (grid, carts) = setup("inputs/13.txt");
    println!("Part 1: {:?}", part1(&grid, &carts));
    println!("Part 2: {:?}", part2(&grid, &carts));
}

fn part1(grid: &[Vec<TrackType>], initial_carts: &[Cart]) -> (usize, usize) {
    let num_carts = initial_carts.len();
    let mut carts = initial_carts.to_vec();
    loop {
        carts.sort_unstable_by_key(|k| (k.row, k.col));
        for idx in 0..num_carts {
            let mut k = &mut carts[idx];
            let track = &grid[k.row][k.col];
            move_cart(&mut k, track);
            for j in 0..num_carts {
                if idx == j {
                    continue;
                }
                if did_crash(&carts[idx], &carts[j]) {
                    return (carts[idx].col, carts[idx].row);
                }
            }
        }
    }
}

fn part2(grid: &[Vec<TrackType>], initial_carts: &[Cart]) -> (usize, usize) {
    let mut carts = initial_carts.to_vec();
    carts.sort_unstable_by_key(|k| (k.row, k.col));
    loop {
        let mut had_crash = false;
        let num_carts = carts.len();
        for idx in 0..num_carts {
            let mut k = &mut carts[idx];
            let track = &grid[k.row][k.col];
            move_cart(&mut k, track);
            for j in 0..num_carts {
                if idx == j {
                    continue;
                }
                if did_crash(&carts[idx], &carts[j]) {
                    carts[j].alive = false;
                    carts[idx].alive = false;
                    had_crash = true;
                }
            }
        }
        carts.sort_unstable_by_key(|k| (!k.alive, k.row, k.col));
        if had_crash {
            let (num_alive, _) = carts.iter().enumerate().find(|(_, k)| !k.alive).unwrap();
            if num_alive < 2 {
                return (carts[0].col, carts[0].row);
            }
            carts.truncate(num_alive);
        }
    }
}

fn did_crash(k1: &Cart, k2: &Cart) -> bool {
    k1.row == k2.row && k1.col == k2.col
}

fn move_cart(cart: &mut Cart, track: &TrackType) {
    cart.facing = match track {
        TrackType::Straight => cart.facing,
        TrackType::CurveLeft => curve_left(cart.facing),
        TrackType::CurveRight => curve_right(cart.facing),
        TrackType::Intersection => {
            cart.num_intersections += 1;
            match cart.num_intersections % 3 {
                1 => turn_left(cart.facing),
                2 => cart.facing,
                0 => turn_right(cart.facing),
                _ => panic!("Bad cart: {:?}", cart),
            }
        }
        TrackType::Empty => panic!("Cart went off the rails: {:?}", cart),
    };
    match cart.facing {
        Direction::North => {
            cart.row -= 1;
        }
        Direction::South => {
            cart.row += 1;
        }
        Direction::East => {
            cart.col += 1;
        }
        Direction::West => {
            cart.col -= 1;
        }
    };
}

fn setup(path: &str) -> (Vec<Vec<TrackType>>, Vec<Cart>) {
    let data = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let grid = lines
        .iter()
        .map(|l| l.chars().map(parse_track).collect())
        .collect();
    let mut carts = vec![];
    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if let Some(cart) = parse_cart(row, col, c) {
                carts.push(cart);
            }
        }
    }
    (grid, carts)
}

#[derive(Debug, PartialEq)]
enum TrackType {
    Straight,
    CurveLeft,
    CurveRight,
    Intersection,
    Empty,
}

fn parse_track(c: char) -> TrackType {
    match c {
        '|' => TrackType::Straight,
        '-' => TrackType::Straight,
        '>' => TrackType::Straight,
        '<' => TrackType::Straight,
        '^' => TrackType::Straight,
        'v' => TrackType::Straight,
        '\\' => TrackType::CurveLeft,
        '/' => TrackType::CurveRight,
        '+' => TrackType::Intersection,
        _ => TrackType::Empty,
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn reverse(facing: Direction) -> Direction {
    match facing {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn turn_left(facing: Direction) -> Direction {
    match facing {
        Direction::North => Direction::West,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
        Direction::West => Direction::South,
    }
}

fn turn_right(facing: Direction) -> Direction {
    reverse(turn_left(facing))
}

fn curve_left(facing: Direction) -> Direction {
    match facing {
        Direction::North => Direction::West,
        Direction::South => Direction::East,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
    }
}

fn curve_right(facing: Direction) -> Direction {
    reverse(curve_left(facing))
}

#[derive(Debug, Clone, Copy)]
struct Cart {
    row: usize,
    col: usize,
    num_intersections: usize,
    facing: Direction,
    alive: bool,
}

fn parse_cart(row: usize, col: usize, c: char) -> Option<Cart> {
    let facing = match c {
        '>' => Direction::East,
        '<' => Direction::West,
        '^' => Direction::North,
        'v' => Direction::South,
        _ => {
            return None;
        }
    };
    Some(Cart {
        row,
        col,
        num_intersections: 0,
        facing,
        alive: true,
    })
}
