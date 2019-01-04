use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn run() {
    let (segs, a, b) = setup("inputs/17.txt");
    let filled = fill_with_water(&segs, a, b);
    let num_filled = filled[segs.len()..].iter().map(|s| s.num_squares()).sum();
    println!("Part 1: {}", part1(&filled, a, b, num_filled));
    println!("Part 2: {}", num_filled);
}

fn fill_with_water(segments: &[LineSegment], min_y: usize, max_y: usize) -> Vec<LineSegment> {
    let mut segs = segments.to_vec();
    let mut start_pts = vec![(500, min_y - 1)];
    let mut checked = HashSet::new();
    while let Some(pt) = start_pts.pop() {
        if !checked.insert(pt) {
            continue;
        }
        if in_clay(pt.0, pt.1, &segs) {
            continue;
        }
        let start_x = pt.0;
        let mut start_y = pt.1;
        loop {
            let result = try_fill((start_x, start_y), &segs, max_y);
            if let Some(seg) = result.0 {
                // it's a fill line
                start_y = seg.y_min - 1;
                segs.push(seg);
                continue;
            }
            if let Some(left) = result.1 {
                start_pts.push(left);
            }
            if let Some(right) = result.2 {
                start_pts.push(right);
            }
            break;
        }
    }
    segs
}

fn part1(filled_segs: &[LineSegment], min_y: usize, max_y: usize, num_at_rest: usize) -> usize {
    // Do a final flow count
    let mut seen = HashSet::new();
    flow_count(500, min_y - 1, &filled_segs, max_y, &mut seen);
    // Discard the starting point at (500, 0) that's included in `seen`.
    num_at_rest + seen.len() - 1
}

fn flow_count(
    x: usize,
    y: usize,
    segments: &[LineSegment],
    max_y: usize,
    seen: &mut HashSet<(usize, usize)>,
) {
    // avoid double-counting paths
    if !seen.insert((x, y)) {
        return;
    }
    if let Some(yy) = drop_down(&(x, y), &segments, max_y) {
        for i in y..=yy {
            seen.insert((x, i));
        }
        // spread left + right
        let mut xx = x - 1;
        while in_sand(xx, yy, &segments) {
            if in_clay(xx, yy + 1, &segments) {
                seen.insert((xx, yy));
                xx -= 1;
            } else {
                flow_count(xx, yy, &segments, max_y, seen);
                break;
            }
        }
        xx = x + 1;
        while in_sand(xx, yy, &segments) {
            if in_clay(xx, yy + 1, &segments) {
                seen.insert((xx, yy));
                xx += 1;
            } else {
                flow_count(xx, yy, &segments, max_y, seen);
                break;
            }
        }
    } else {
        // fell straight off
        for i in y..=max_y {
            seen.insert((x, i));
        }
    }
}

fn try_fill(
    pt: (usize, usize),
    segments: &[LineSegment],
    max_y: usize,
) -> (
    Option<LineSegment>,
    Option<(usize, usize)>,
    Option<(usize, usize)>,
) {
    if let Some(y) = drop_down(&pt, &segments, max_y) {
        // hit something, spread left + right
        let mut x = pt.0 - 1;
        let mut left = None;
        let mut right = None;
        while in_sand(x, y, &segments) {
            if in_clay(x, y + 1, &segments) {
                x -= 1;
            } else {
                left = Some((x, y));
                break;
            }
        }
        let x_min = x + 1;
        x = pt.0 + 1;
        while in_sand(x, y, &segments) {
            if in_clay(x, y + 1, &segments) {
                x += 1;
            } else {
                right = Some((x, y));
                break;
            }
        }
        let x_max = x - 1;
        if left == None && right == None {
            return (
                Some(LineSegment {
                    x_min,
                    x_max,
                    y_min: y,
                    y_max: y,
                }),
                left,
                right,
            );
        }
        return (None, left, right);
    }
    // fell out of the world
    (None, None, None)
}

fn drop_down(xy: &(usize, usize), segments: &[LineSegment], max_y: usize) -> Option<usize> {
    let x = xy.0;
    let mut y = xy.1;
    while in_sand(x, y, &segments) {
        if y >= max_y {
            return None;
        }
        y += 1;
    }
    if y == xy.1 {
        // TODO: figure out why this happens
        panic!("drop_down started in a wall, at {:?}", xy);
    }
    Some(y - 1)
}

fn in_clay(x: usize, y: usize, segments: &[LineSegment]) -> bool {
    segments.iter().any(|s| s.contains(x, y))
}
fn in_sand(x: usize, y: usize, segments: &[LineSegment]) -> bool {
    segments.iter().all(|s| !s.contains(x, y))
}

fn setup(path: &str) -> (Vec<LineSegment>, usize, usize) {
    let data = fs::read_to_string(path).unwrap();
    let segs: Vec<LineSegment> = data.lines().map(|l| l.parse().unwrap()).collect();
    let min_y = segs.iter().map(|s| s.y_min).min().unwrap();
    let max_y = segs.iter().map(|s| s.y_max).max().unwrap();
    (segs, min_y, max_y)
}

#[derive(Debug, Clone)]
struct LineSegment {
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl LineSegment {
    fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
    fn num_squares(&self) -> usize {
        self.x_max - self.x_min + self.y_max - self.y_min + 1
    }
}

impl FromStr for LineSegment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        let lhs_val = parts[0][2..].parse()?;
        let lhs_id = &parts[0][..1];
        let range_parts: Vec<&str> = parts[1][2..].split("..").collect();
        let range_min = range_parts[0].parse()?;
        let range_max = range_parts[1].parse()?;
        Ok(if lhs_id == "x" {
            LineSegment {
                x_min: lhs_val,
                x_max: lhs_val,
                y_min: range_min,
                y_max: range_max,
            }
        } else {
            LineSegment {
                y_min: lhs_val,
                y_max: lhs_val,
                x_min: range_min,
                x_max: range_max,
            }
        })
    }
}
