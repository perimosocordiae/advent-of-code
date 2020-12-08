// Shared code between days.
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_use]
extern crate scan_fmt;

pub mod bootcode;

pub fn read_string(name: &str) -> String {
    let path = Path::new(name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    }
    s
}

pub fn read_integers(name: &str) -> Vec<i64> {
    let data = read_string(name);
    data.lines().map(|x| x.parse::<i64>().unwrap()).collect()
}

pub fn read_bootcode(name: &str) -> Vec<bootcode::Instruction> {
    let data = read_string(name);
    data.lines().map(|line| line.parse().unwrap()).collect()
}
