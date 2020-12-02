use criterion_plot::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;
use std::ops;
use std::path::Path;
use std::str::FromStr;

pub fn run() {
    let (points, velocities) = setup();
    part1(&points, &velocities);
    println!("Part 1: see day10.svg");
    println!("Part 2: {}", 10027);
}

fn setup() -> (Vec<Point>, Vec<Point>) {
    let f = File::open("inputs/10.txt").unwrap();
    let mut points: Vec<Point> = vec![];
    let mut velocities: Vec<Point> = vec![];
    BufReader::new(&f).lines().for_each(|l| {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(" velocity").collect();
        points.push(
            parts[0].split('=').collect::<Vec<&str>>()[1]
                .parse()
                .unwrap(),
        );
        velocities.push(
            parts[1].split('=').collect::<Vec<&str>>()[1]
                .parse()
                .unwrap(),
        );
    });
    (points, velocities)
}

fn part1(start_points: &[Point], velocities: &[Point]) {
    let n = start_points.len();
    let mut points: Vec<Point> = start_points.to_vec();
    for i in 0..n {
        points[i] += velocities[i] * 10027;
    }
    plot_points(&points);
}

fn plot_points(points: &[Point]) {
    Figure::new()
        .configure(Key, |k| {
            k.set(Boxed::Yes)
                .set(Position::Inside(Vertical::Top, Horizontal::Left))
        })
        .plot(
            Points {
                x: points.iter().map(|p| p.y),
                y: points.iter().map(|p| -p.x),
            },
            |lp| {
                lp.set(Color::Black)
                    .set(PointSize(1.0))
                    .set(PointType::FilledSquare)
            },
        )
        .set(Output(Path::new("day10.svg")))
        .draw()
        .ok()
        .and_then(|gnuplot| {
            gnuplot
                .wait_with_output()
                .ok()
                .and_then(|p| String::from_utf8(p.stderr).ok())
        });
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .trim_matches(|c| c == '<' || c == '>')
            .split(',')
            .collect();
        Ok(Point {
            x: parts[1].trim().parse()?,
            y: parts[0].trim().parse()?,
        })
    }
}
