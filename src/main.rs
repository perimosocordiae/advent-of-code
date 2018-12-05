use std::env;
use std::process;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: u8;
    if args.len() == 2 {
      day_num = args[1].parse().unwrap_or_else(|err|{
        println!("Parse error: {}", err);
        process::exit(1);
      });
    } else {
      println!("Usage: {} <day>", &args[0]);
      process::exit(1);
    }
    match day_num {
      1 => day1::run(),
      2 => day2::run(),
      3 => day3::run(),
      4 => day4::run(),
      _ => {
        println!("Invalid day: {}", day_num);
        process::exit(1);
      }
    }
}
