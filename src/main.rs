use std::fmt::Display;
use std::str::FromStr;

mod day1;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let n: &str = &std::env::args().skip(1).next().expect("Must provide number of puzzle to run");
    println!("{}", match n {
        "1-1" => execute("1.txt", day1::run1),
        "1-2" => execute("1.txt", day1::run2),
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
