use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn run() {
    let (map, units) = setup("inputs/15.txt");
    println!("Part 1: {}", part1(&map, &units));
    println!("Part 2: {}", part2(&map, &units));
}

fn part1(grid: &[Vec<Cell>], initial_units: &[Unit]) -> i32 {
    let (num_rounds, units) = run_battle(&grid, &initial_units, 3, false);
    let total_hp: i32 = units.iter().filter(|u| u.hp > 0).map(|u| u.hp).sum();
    num_rounds * total_hp
}

fn part2(grid: &[Vec<Cell>], initial_units: &[Unit]) -> i32 {
    // TODO: binary search here?
    for elf_attack in 4..100 {
        let (num_rounds, units) = run_battle(&grid, &initial_units, elf_attack, false);
        if units
            .iter()
            .filter(|u| u.team == Team::Elf)
            .all(|u| u.hp > 0)
        {
            let total_hp: i32 = units.iter().filter(|u| u.hp > 0).map(|u| u.hp).sum();
            return num_rounds * total_hp;
        }
    }
    0
}

fn run_battle(
    grid: &[Vec<Cell>],
    initial_units: &[Unit],
    elf_attack: i32,
    debug: bool,
) -> (i32, Vec<Unit>) {
    let num_units = initial_units.len();
    let mut units = initial_units.to_vec();
    let mut num_rounds = 0i32;
    while num_rounds < 99999 {
        units.sort_unstable_by_key(|u| (u.hp <= 0, u.row, u.col));
        if debug {
            display_state(&grid, &units);
        }
        for idx in 0..num_units {
            let unit = &units[idx];
            if unit.hp <= 0 {
                continue;
            }
            let targets_idx: Vec<usize> = units
                .iter()
                .enumerate()
                .filter(|(_, u)| u.hp > 0 && u.team != unit.team)
                .map(|(i, _)| i)
                .collect();
            if targets_idx.is_empty() {
                return (num_rounds, units);
            }
            let attack_power = match unit.team {
                Team::Elf => elf_attack,
                Team::Goblin => 3,
            };
            if let Some(tidx) = try_attack(&units, idx, &targets_idx) {
                units[tidx].hp -= attack_power;
            } else if let Some((r, c)) = find_move(&grid, &units, idx, &targets_idx) {
                // move to (r,c)
                units[idx].row = r;
                units[idx].col = c;
                if let Some(tidx) = try_attack(&units, idx, &targets_idx) {
                    units[tidx].hp -= attack_power;
                }
            }
        }
        if num_units == 0 {
            return (num_rounds, units);
        }
        num_rounds += 1;
    }
    (0, units)
}

fn is_open(row: i32, col: i32, grid: &[Vec<Cell>], units: &[Unit]) -> bool {
    if grid[row as usize][col as usize] != Cell::Open {
        return false;
    }
    !units
        .iter()
        .filter(|u| u.hp > 0)
        .any(|u| u.row == row && u.col == col)
}

fn next_to(a: &Unit, b: &Unit) -> bool {
    let dist = (a.row - b.row).abs() + (a.col - b.col).abs();
    dist == 1
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

fn display_state(grid: &[Vec<Cell>], units: &[Unit]) {
    for (row, line) in grid.iter().enumerate() {
        let mut c: Vec<char> = line
            .iter()
            .map(|cell| match cell {
                Cell::Wall => '#',
                _ => '.',
            })
            .collect();
        let mut pieces = vec![];
        for u in units.iter() {
            if u.hp > 0 && u.row == row as i32 {
                let icon = match u.team {
                    Team::Elf => 'E',
                    _ => 'G',
                };
                c[u.col as usize] = icon;
                pieces.push(format!("{}({})", icon, u.hp).to_string());
            }
        }
        let s: String = c.into_iter().collect();
        let t: String = pieces.join(", ");
        println!("{}   {}", s, t);
    }
    println!("");
}

fn try_attack(units: &[Unit], idx: usize, targets_idx: &[usize]) -> Option<usize> {
    let unit = &units[idx];
    // Targets were in sorted order originally, but may have moved since then.
    targets_idx
        .iter()
        .filter(|&i| next_to(&unit, &units[*i]))
        .cloned()
        .min_by_key(|&i| (units[i].hp, units[i].row, units[i].col))
}

fn find_move(
    grid: &[Vec<Cell>],
    units: &[Unit],
    start: usize,
    targets_idx: &[usize],
) -> Option<(i32, i32)> {
    let mut target_locs = HashSet::new();
    for tidx in targets_idx.iter() {
        let t = &units[*tidx];
        for c in next_moves(&grid, &units, t.row, t.col) {
            target_locs.insert(c);
        }
    }
    if target_locs.is_empty() {
        return None;
    }
    let unit = &units[start];
    let moves = next_moves(&grid, &units, unit.row, unit.col);
    let mut best_move = None;
    let mut best_key = (99999, (0, 0));
    for m in moves {
        if let Some(key) = shortest_path(&grid, &units, m, &target_locs) {
            if best_key > key {
                best_key = key;
                best_move = Some(m);
            }
        }
    }
    best_move
}

fn shortest_path(
    grid: &[Vec<Cell>],
    units: &[Unit],
    start_loc: (i32, i32),
    target_locs: &HashSet<(i32, i32)>,
) -> Option<(i32, (i32, i32))> {
    let mut dist = HashMap::new();
    dist.insert(start_loc, 0);
    let mut queue = BinaryHeap::new();
    queue.push((0, 0, start_loc));
    let large_cost = 99999;
    let mut max_cost = large_cost;
    while let Some((neg_cost, _, loc)) = queue.pop() {
        let cost = -neg_cost;
        if cost > *dist.get(&loc).unwrap_or(&large_cost) {
            continue;
        }
        if target_locs.contains(&loc) && max_cost > cost {
            max_cost = cost;
        }
        if cost >= max_cost {
            continue;
        }
        let children = next_moves(&grid, &units, loc.0, loc.1);
        for (i, c) in children.into_iter().enumerate() {
            let c_cost = cost + 1;
            let d = *dist.get(&c).unwrap_or(&large_cost);
            if c_cost < d {
                queue.push((-c_cost, 4 - i, c));
                dist.insert(c, c_cost);
            }
        }
    }
    if max_cost == large_cost {
        return None;
    }
    let target_loc = *target_locs
        .iter()
        .min_by_key(|&loc| (*dist.get(&loc).unwrap_or(&large_cost), loc))
        .expect("No target_locs");
    Some((max_cost, target_loc))
}

fn next_moves(grid: &[Vec<Cell>], units: &[Unit], r: i32, c: i32) -> Vec<(i32, i32)> {
    let mut children = vec![];
    if is_open(r - 1, c, &grid, &units) {
        children.push((r - 1, c));
    }
    if is_open(r, c - 1, &grid, &units) {
        children.push((r, c - 1));
    }
    if is_open(r, c + 1, &grid, &units) {
        children.push((r, c + 1));
    }
    if is_open(r + 1, c, &grid, &units) {
        children.push((r + 1, c));
    }
    children
}
