use std::fs;
//use std::collections::HashSet;

pub fn run() {
    let (map, units) = setup("inputs/15-small.txt");
    println!("Part 1: {}", part1(&map, &units));
    //println!("Part 2: {}", part2(&map, &units));
}

fn part1(grid: &[Vec<Cell>], initial_units: &[Unit]) -> i32 {
    let mut num_units = initial_units.len();
    let mut units = initial_units.to_vec();
    println!("{:?}", units);
    println!("{:?}", grid);
    let mut num_rounds = 0i32;
    loop {
        units.sort_unstable_by_key(|u| (u.hp <= 0, u.row, u.col));
        for idx in 0..num_units {
            let unit = &units[idx];
            if unit.hp <= 0 {
                num_units = idx;
                break;
            }
            let targets: Vec<&Unit> = units
                .iter()
                .filter(|u| u.hp > 0 && u.team != unit.team)
                .collect();
            if targets.is_empty() {
                return combat_end(num_rounds, &units);
            }
            let in_range: Vec<&Unit> = targets
                .iter()
                .filter(|u| next_to(&unit, &u))
                .map(|u| *u)
                .collect();
            if in_range.is_empty() {
                let mut open_dests = vec![];
                for t in targets {
                    if is_open(t.row - 1, t.col, &grid, &units) {
                        open_dests.push((t.row - 1, t.col));
                    }
                    if is_open(t.row, t.col - 1, &grid, &units) {
                        open_dests.push((t.row, t.col - 1));
                    }
                    if is_open(t.row, t.col + 1, &grid, &units) {
                        open_dests.push((t.row, t.col + 1));
                    }
                    if is_open(t.row + 1, t.col, &grid, &units) {
                        open_dests.push((t.row + 1, t.col));
                    }
                }
                if open_dests.is_empty() {
                    continue;
                }
                let mut tiers = vec![vec![(unit.row, unit.col)]];
                loop {
                    let last_tier = &tiers[tiers.len() - 1];
                    let mut next_tier = vec![];
                    for &(r, c) in last_tier.iter() {
                        if is_open(r - 1, c, &grid, &units) {
                            next_tier.push((r - 1, c));
                        }
                        if is_open(r, c - 1, &grid, &units) {
                            next_tier.push((r, c - 1));
                        }
                        if is_open(r, c + 1, &grid, &units) {
                            next_tier.push((r, c + 1));
                        }
                        if is_open(r + 1, c, &grid, &units) {
                            next_tier.push((r + 1, c));
                        }
                    }
                    if next_tier.is_empty() {
                        break;
                    }
                    let nearest: Vec<(i32, i32)> = open_dests
                        .iter()
                        .filter(|&d| next_tier.iter().find(|&x| x == d).is_some())
                        .cloned()
                        .collect();
                    if nearest.is_empty() {
                        tiers.push(next_tier);
                    } else {
                        let chosen = nearest.iter().min().unwrap();
                        let step = tiers[1]
                            .iter()
                            .min_by_key(|loc| (loc.0 - chosen.0).abs() + (loc.1 - chosen.1).abs())
                            .unwrap();
                        // TODO fixme
                        units[idx].row = step.0;
                        units[idx].col = step.1;
                        break;
                    }
                }
                let unit = &units[idx];
                let in_range: Vec<&Unit> = targets
                    .iter()
                    .filter(|u| next_to(&unit, &u))
                    .map(|u| *u)
                    .collect();
                if !in_range.is_empty() {
                    let target = *in_range.iter().min_by_key(|u| u.hp).unwrap();
                    let tidx = units.iter().position(|u| u == target).unwrap();
                    units[tidx].hp -= 3;
                }
            } else {
                let target = *in_range.iter().min_by_key(|u| u.hp).unwrap();
                let tidx = units.iter().position(|u| u == target).unwrap();
                units[tidx].hp -= 3;
            }
        }
        if num_units == 0 {
            return 0;
        }
        num_rounds += 1;
    }
}

fn is_open(row: i32, col: i32, grid: &[Vec<Cell>], units: &[Unit]) -> bool {
    if grid[row as usize][col as usize] != Cell::Open {
        return false;
    }
    units
        .iter()
        .filter(|u| u.hp > 0)
        .all(|u| u.row != row && u.col != col)
}

fn next_to(a: &Unit, b: &Unit) -> bool {
    let dist = (a.row - b.row).abs() + (a.col - b.col).abs();
    return dist == 1;
}

fn combat_end(num_rounds: i32, units: &[Unit]) -> i32 {
    let total_hp: i32 = units.iter().filter(|u| u.hp >= 0).map(|u| u.hp).sum();
    num_rounds * total_hp
}

fn setup(path: &str) -> (Vec<Vec<Cell>>, Vec<Unit>) {
    let data = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let grid = lines
        .iter()
        .map(|l| l.chars().map(parse_cell).collect())
        .collect();
    let mut units = vec![];
    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if let Some(unit) = parse_unit(row, col, c) {
                units.push(unit);
            }
        }
    }
    (grid, units)
}

#[derive(Debug, PartialEq)]
enum Cell {
    Wall,
    Open,
}

fn parse_cell(c: char) -> Cell {
    match c {
        '#' => Cell::Wall,
        _ => Cell::Open,
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Team {
    Elf,
    Goblin,
}

#[derive(Debug, PartialEq, Clone)]
struct Unit {
    row: i32,
    col: i32,
    team: Team,
    hp: i32,
}

fn parse_unit(row: usize, col: usize, c: char) -> Option<Unit> {
    let team = match c {
        'G' => Team::Goblin,
        'E' => Team::Elf,
        _ => {
            return None;
        }
    };
    Some(Unit {
        row: row as i32,
        col: col as i32,
        team,
        hp: 200,
    })
}
