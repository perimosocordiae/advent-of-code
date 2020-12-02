use chrono::NaiveDateTime;
use chrono::Timelike;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn setup() -> Vec<GuardHistory> {
    let f = File::open("inputs/04.txt").unwrap();
    let mut data: Vec<LogMessage> = BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    data.sort_unstable_by_key(|x| x.when);
    let mut history = HashMap::new();
    let mut accum = history
        .entry(data[0].guard)
        .or_insert_with(|| GuardHistory::new(data[0].guard));
    let mut start_sleep: usize = 0;
    for log in data {
        match log.event {
            EventType::START => {
                accum = history
                    .entry(log.guard)
                    .or_insert_with(|| GuardHistory::new(log.guard));
            }
            EventType::SLEEP => {
                start_sleep = log.when.time().minute() as usize;
            }
            EventType::WAKE => {
                let stop_sleep = log.when.time().minute() as usize;
                for i in start_sleep..stop_sleep {
                    accum.minutes[i] += 1;
                }
            }
        }
    }
    history.into_iter().map(|(_, v)| v).collect()
}

fn part1(data: &[GuardHistory]) -> i32 {
    let best = data
        .iter()
        .max_by_key(|&g| g.minutes.iter().sum::<i32>())
        .unwrap();
    solution_code(best)
}

fn part2(data: &[GuardHistory]) -> i32 {
    let best = data.iter().max_by_key(|&g| g.minutes.iter().max()).unwrap();
    solution_code(best)
}

fn solution_code(gh: &GuardHistory) -> i32 {
    i32::from(gh.id) * argmax(&gh.minutes) as i32
}

fn argmax<T: Ord>(values: &[T]) -> usize {
    let (idx, _) = values.iter().enumerate().max_by_key(|(_, x)| *x).unwrap();
    idx
}

struct GuardHistory {
    id: u16,
    minutes: [i32; 60],
}

impl GuardHistory {
    fn new(id: u16) -> GuardHistory {
        GuardHistory {
            id,
            minutes: [0i32; 60],
        }
    }
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
            _ => return Err(ParseLogError {}.into()),
        }
        Ok(msg)
    }
}
