extern crate chrono;
extern crate ndarray;
extern crate petgraph;
use std::env;
use std::process;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <day>", &args[0]);
        process::exit(1);
    }
    args[1..]
        .iter()
        .map(|arg| {
            arg.parse().unwrap_or_else(|err| {
                println!("Bad argument '{}': {}", arg, err);
                process::exit(1);
            })
        })
        .for_each(run_day);
}

fn run_day(day_num: usize) {
    println!(" ### Day {} ###", day_num);
    match day_num {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        _ => {
            println!("Invalid day: {}", day_num);
            process::exit(1);
        }
    }
    println!();
}
