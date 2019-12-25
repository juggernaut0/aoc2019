#[macro_use] extern crate itertools;

use std::fmt::Debug;
use std::str::FromStr;

use clap::{App, Arg};
use log::Level;

mod intcode;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
        "3-1" => execute("3.txt", day3::run1),
        "3-2" => execute("3.txt", day3::run2),
        "4-1" => day4::run1(),
        "4-2" => day4::run2(),
        "5-1" => execute("5.txt", day5::run1),
        "5-2" => execute("5.txt", day5::run2),
        "6-1" => execute("6.txt", day6::run1),
        "6-2" => execute("6.txt", day6::run2),
        "7-1" => execute("7.txt", day7::run1),
        "7-2" => execute("7.txt", day7::run2),
        "8-1" => execute("8.txt", day8::run1),
        "8-2" => execute("8.txt", day8::run2),
        "9-1" => execute("9.txt", day9::run1),
        "9-2" => execute("9.txt", day9::run2),
        "10-1" => execute("10.txt", day10::run1),
        "10-2" => execute("10.txt", day10::run2),
        "11-1" => execute("11.txt", day11::run1),
        "11-2" => execute("11.txt", day11::run2),
        "12-1" => execute("12.txt", day12::run1),
        "12-2" => execute("12.txt", day12::run2),
        "13-1" => execute("13.txt", day13::run1),
        "13-2" => execute("13.txt", day13::run2),
        "14-1" => execute("14.txt", day14::run1),
        "14-2" => execute("14.txt", day14::run2),
        "15-1" => execute("15.txt", day15::run1),
        "15-2" => execute("15.txt", day15::run2),
        "16-1" => execute("16.txt", day16::run1),
        "16-2" => execute("16.txt", day16::run2),
        "17-1" => execute("17.txt", day17::run1),
        "17-2" => execute("17.txt", day17::run2),
        "18-1" => execute("18.txt", day18::run1),
        "18-2" => execute("18-2.txt", day18::run2),
        "19-1" => execute("19.txt", day19::run1),
        "19-2" => execute("19.txt", day19::run2),
        "20-1" => execute("20.txt", day20::run1),
        "20-2" => execute("20.txt", day20::run2),
        "21-1" => execute("21.txt", day21::run1),
        "21-2" => execute("21.txt", day21::run2),
        "22-1" => execute("22.txt", day22::run1),
        "22-2" => execute("22.txt", day22::run2),
        "23-1" => execute("23.txt", day23::run1),
        "23-2" => execute("23.txt", day23::run2),
        "24-1" => execute("24.txt", day24::run1),
        "24-2" => execute("24.txt", day24::run2),
        "25-1" => execute("25.txt", day25::run1),
        "25-2" => execute("25.txt", day25::run2),
        _ => "No puzzle with that number".to_string()
    })
}

fn execute<F, T: FromStr, R: Debug>(input_path: &str, f: F) -> String
        where F: FnOnce(Vec<T>) -> R, T::Err : Debug
{
    let input = std::fs::read_to_string(format!("input/{}", input_path)).unwrap();
    format!("{:#?}", f(parse_input(&input)))
}

pub fn parse_input<T>(input: &str) -> Vec<T> where T: FromStr, T::Err : Debug {
    input.lines()
        .map(|it| it.parse().expect(&format!("Could not parse input: {}", it)))
        .collect()
}
