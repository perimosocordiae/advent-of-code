use advent2020::*;
use std::str::FromStr;

fn main() {
    let data = read_string("inputs/12.full");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[test]
fn part1_small() {
    let data = read_string("inputs/12.test");
    assert_eq!(part1(&data), 25);
}

#[test]
fn part2_small() {
    let data = read_string("inputs/12.test");
    assert_eq!(part2(&data), 286);
}

fn part1(input: &str) -> i32 {
    let mut pos = [0, 0];
    let mut facing = 0;
    for step in input.lines().map(|p| p.parse::<NavStep>().unwrap()) {
        match step {
            NavStep::Forward(x) => {
                move_direction(&facing_step(facing, x), &mut pos);
            }
            NavStep::Left(x) => {
                facing += x;
                if facing >= 360 {
                    facing -= 360;
                }
            }
            NavStep::Right(x) => {
                facing -= x;
                if facing < 0 {
                    facing += 360;
                }
            }
            _ => {
                move_direction(&step, &mut pos);
            }
        }
    }
    pos[0].abs() + pos[1].abs()
}

fn part2(input: &str) -> i32 {
    let mut pos = [0, 0];
    let mut waypoint = [1, 10];
    for step in input.lines().map(|p| p.parse::<NavStep>().unwrap()) {
        match step {
            NavStep::Forward(x) => {
                pos[0] += waypoint[0] * x;
                pos[1] += waypoint[1] * x;
            }
            NavStep::Left(x) => {
                rotate_waypoint(x, &mut waypoint);
            }
            NavStep::Right(x) => {
                rotate_waypoint(360 - x, &mut waypoint);
            }
            _ => {
                move_direction(&step, &mut waypoint);
            }
        }
    }
    pos[0].abs() + pos[1].abs()
}

fn rotate_waypoint(angle: i32, pos: &mut [i32]) {
    match angle {
        90 => {
            // n4 e10 -> n10 e-4
            pos.swap(0, 1);
            pos[1] *= -1;
        }
        180 => {
            // n4 e10 -> n-4 e-10
            pos[0] *= -1;
            pos[1] *= -1;
        }
        270 => {
            // n4 e10 -> n-10 e4
            pos.swap(0, 1);
            pos[0] *= -1;
        }
        _ => panic!("Bad angle: {}", angle),
    }
}

fn facing_step(facing: i32, amount: i32) -> NavStep {
    match facing {
        0 => NavStep::East(amount),
        90 => NavStep::North(amount),
        180 => NavStep::West(amount),
        270 => NavStep::South(amount),
        _ => panic!("Bad facing: {}", facing),
    }
}

fn move_direction(dir: &NavStep, pos: &mut [i32]) {
    match dir {
        NavStep::North(x) => {
            pos[0] += x;
        }
        NavStep::South(x) => {
            pos[0] -= x;
        }
        NavStep::East(x) => {
            pos[1] += x;
        }
        NavStep::West(x) => {
            pos[1] -= x;
        }
        _ => panic!("Bad direction: {:?}", dir),
    }
}

#[derive(Debug, Clone, Copy)]
enum NavStep {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for NavStep {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amt: i32 = s[1..].parse()?;
        match s.chars().next().unwrap() {
            'N' => Ok(NavStep::North(amt)),
            'S' => Ok(NavStep::South(amt)),
            'E' => Ok(NavStep::East(amt)),
            'W' => Ok(NavStep::West(amt)),
            'L' => Ok(NavStep::Left(amt)),
            'R' => Ok(NavStep::Right(amt)),
            'F' => Ok(NavStep::Forward(amt)),
            _ => panic!("Unknown NavStep direction: {:?}", s),
        }
    }
}
