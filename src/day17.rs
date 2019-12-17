use crate::intcode::*;
use std::rc::Rc;
use std::collections::HashMap;
use itertools::Itertools;

pub fn run1(input: Vec<String>) -> usize {
    let mut comp = Computer::new(parse_program(&input[0]));
    let output = Stream::new_wrapped();
    comp.set_output(Some(Rc::clone(&output)));
    comp.execute();
    let view: String = output.borrow_mut().read_all().iter().map(|&a| char::from(a as u8)).collect();
    let map = to_map(&view);
    let width = map.keys().map(|&(x, _)| x).max().unwrap() + 1;
    let height = map.keys().map(|&(_, y)| y).max().unwrap() + 1;
    let mut sum = 0;
    for (x, y) in (1..width-1).cartesian_product(1..height-1) {
        if map[&(x, y)] != '.' && map[&(x-1, y)] != '.' && map[&(x+1, y)] != '.' && map[&(x, y-1)] != '.' && map[&(x, y+1)] != '.' {
            log::debug!("intersection at ({}, {})", x, y);
            sum += x * y
        }
    }
    println!("{}", view);
    sum
}

pub fn run2(input: Vec<String>) -> i64 {
    let mut program = parse_program(&input[0]);
    program[0] = 2;
    let mut comp = Computer::new(program);
    let input = Stream::new_wrapped();
    let output = Stream::new_wrapped();
    comp.set_input(Some(Rc::clone(&input)));
    comp.set_output(Some(Rc::clone(&output)));

    let interactive = true;
    let commands = "A,B,A,B,C,A,C,A,C,B
R,12,L,8,L,4,L,4
L,8,R,6,L,6
L,8,L,4,R,12,L,6,L,4
";
    input.borrow_mut().write_all(&to_input(commands));
    input.borrow_mut().write_all(&to_input(if interactive { "y\n" } else { "n\n" }));
    comp.execute();

    let raw_output = output.borrow_mut().read_all();
    let view: String = raw_output[0..raw_output.len()-1].iter().map(|&a| char::from(a as u8)).collect();
    println!("{}", view); // TODO show this in a more usable (interactive?) way
    raw_output[raw_output.len()-1]
}

fn to_map(view: &str) -> HashMap<(usize, usize), char> {
    let mut map = HashMap::new();
    for (y, line) in view.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), c);
        }
    }
    map
}

fn to_input(s: &str) -> Vec<i64> {
    s.chars().map(|c| c as u32 as i64).collect()
}
