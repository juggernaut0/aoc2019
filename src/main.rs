#[macro_use] extern crate itertools;

use std::fmt::Display;
use std::str::FromStr;

use clap::{App, Arg};
use log::Level;

mod day1;
mod day2;

fn main() {
    let matches = App::new("aoc2019")
        .about("Advent of Code 2019")
        .arg(Arg::with_name("puzzle")
            .help("Puzzle number, e.g. 2-1 for day 2, puzzle 1")
            .required(true))
        .arg(Arg::with_name("log_level")
            .long("level")
            .help("Logging level")
            .takes_value(true)
            .default_value("info"))
        .get_matches();

    let level_match: &str = &matches.value_of("log_level").map(|it| it.to_lowercase()).unwrap();
    let log_level = match level_match {
        "trace" => Level::Trace,
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warn,
        "error" => Level::Error,
        _ => Level::Info
    };

    simple_logger::init_with_level(log_level).unwrap();

    println!("{}", match matches.value_of("puzzle").unwrap() {
        "1-1" => execute("1.txt", day1::run1),
        "1-2" => execute("1.txt", day1::run2),
        "2-1" => execute("2.txt", day2::run1),
        "2-2" => execute("2.txt", day2::run2),
        _ => "No puzzle with that number".to_string()
    })
}

fn execute<F, T: FromStr, R: Display>(input_path: &str, f: F) -> String
        where F: FnOnce(Vec<T>) -> R
{
    let input = std::fs::read_to_string("input/".to_string() + input_path)
        .unwrap()
        .lines()
        .map(|it| it.parse().ok().expect(&format!("Could not parse input: {}", it)))
        .collect();
    f(input).to_string()
}
