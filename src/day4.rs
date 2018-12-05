use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use chrono::Timelike;
use std::str::FromStr;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    // println!("Part 2: {}", part2(&data));
}

fn setup() -> Vec<LogMessage> {
    let f = File::open("inputs/04.txt").unwrap();
    let mut data: Vec<LogMessage> = BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    data.sort_unstable_by_key(|x| x.when);
    data
}

fn part1(data: &Vec<LogMessage>) -> i32 {
    let mut hours = HashMap::new();
    let mut accum: &mut[i32] = hours.entry(data[0].guard).or_insert(vec![0i32; 60]);
    let mut start_sleep: usize = 0;
    for log in data.iter() {
        match log.event {
            EventType::START => {
                accum = hours.entry(log.guard).or_insert(vec![0i32; 60]);
            },
            EventType::SLEEP => {
                start_sleep = log.when.time().minute() as usize;
            },
            EventType::WAKE => {
                let stop_sleep = log.when.time().minute() as usize;
                for i in start_sleep..stop_sleep {
                    accum[i] += 1;
                }
            }
        }
    }
    let mut curr_guard: u16 = 0;
    let mut max_slept: i32 = 0;
    for (guard, grid) in hours.iter() {
        let time_slept = grid.iter().sum();
        if time_slept > max_slept {
            curr_guard = *guard;
            max_slept = time_slept;
        }
    }
    let grid = &hours[&curr_guard];
    let maxgrid = grid.iter().max().unwrap();
    let argmax = grid.iter().position(|g| g == maxgrid).unwrap() as i32;
    return curr_guard as i32 * argmax;
}

#[derive(Debug, PartialEq)]
enum EventType {
    START,
    SLEEP,
    WAKE,
}

#[derive(Debug, PartialEq)]
struct LogMessage {
    when: NaiveDateTime,
    guard: u16,
    event: EventType,
}

#[derive(Debug)]
struct ParseLogError;

impl fmt::Display for ParseLogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid log message")
    }
}

impl Error for ParseLogError {
    fn description(&self) -> &str {
        "Invalid log message"
    }
}

impl FromStr for LogMessage {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s[1..].split(']').collect();
        let words: Vec<&str> = parts[1].split_whitespace().collect();
        let mut msg = LogMessage {
            when: NaiveDateTime::parse_from_str(parts[0], "%Y-%m-%d %H:%M")?,
            guard: 0,
            event: EventType::START,
        };
        match words[0] {
            "Guard" => {
                msg.guard = words[1][1..].parse()?;
            }
            "falls" => {
                msg.event = EventType::SLEEP;
            }
            "wakes" => {
                msg.event = EventType::WAKE;
            }
            _ => return Err(ParseLogError{}.into()),
        }
        Ok(msg)
    }
}
