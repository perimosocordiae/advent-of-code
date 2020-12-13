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
    let mut pos = Position { lat: 0, lon: 0 };
    let mut facing = 0;
    for step in input.lines().map(|p| p.parse::<NavStep>().unwrap()) {
        match step {
            NavStep::Forward(x) => {
                pos.move_direction(&facing_step(facing, x));
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
                pos.move_direction(&step);
            }
        }
    }
    pos.l1_norm()
}

fn part2(input: &str) -> i32 {
    let mut pos = Position { lat: 0, lon: 0 };
    let mut waypoint = Position { lat: 1, lon: 10 };
    for step in input.lines().map(|p| p.parse::<NavStep>().unwrap()) {
        match step {
            NavStep::Forward(x) => {
                pos.lat += waypoint.lat * x;
                pos.lon += waypoint.lon * x;
            }
            NavStep::Left(x) => {
                waypoint.rotate(x);
            }
            NavStep::Right(x) => {
                waypoint.rotate(360 - x);
            }
            _ => {
                waypoint.move_direction(&step);
            }
        }
    }
    pos.l1_norm()
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

#[derive(Debug)]
struct Position {
    lat: i32,
    lon: i32,
}

impl Position {
    fn l1_norm(&self) -> i32 {
        self.lat.abs() + self.lon.abs()
    }

    fn move_direction(&mut self, dir: &NavStep) {
        match dir {
            NavStep::North(x) => {
                self.lat += x;
            }
            NavStep::South(x) => {
                self.lat -= x;
            }
            NavStep::East(x) => {
                self.lon += x;
            }
            NavStep::West(x) => {
                self.lon -= x;
            }
            _ => panic!("Bad direction: {:?}", dir),
        }
    }

    fn rotate(&mut self, angle: i32) {
        match angle {
            90 => {
                // n4 e10 -> n10 e-4
                let tmp = -self.lat;
                self.lat = self.lon;
                self.lon = tmp;
            }
            180 => {
                // n4 e10 -> n-4 e-10
                self.lat *= -1;
                self.lon *= -1;
            }
            270 => {
                // n4 e10 -> n-10 e4
                let tmp = -self.lon;
                self.lon = self.lat;
                self.lat = tmp;
            }
            _ => panic!("Bad angle: {}", angle),
        }
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
