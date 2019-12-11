use crate::intcode;
use std::collections::HashMap;
use crate::intcode::{Program, Computer, Stream, ComputerState};

pub fn run1(input: Vec<String>) -> usize {
    let program = intcode::parse_program(&input[0]);
    let mut grid: HashMap<Point, u8> = HashMap::new();
    run_robot(program, &mut grid);
    print_grid(&grid);
    grid.keys().count()
}

pub fn run2(input: Vec<String>) {
    let program = intcode::parse_program(&input[0]);
    let mut grid: HashMap<Point, u8> = HashMap::new();
    grid.insert((0, 0), 1);
    run_robot(program, &mut grid);
    print_grid(&grid);
}

type Point = (i32, i32);
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up, Left, Down, Right
}

fn run_robot(program: Program, grid: &mut HashMap<Point, u8>) {
    let mut pos = (0, 0);
    let mut dir = Direction::Up;

    let mut comp = Computer::new(program);
    comp.set_input(Some(Stream::new_wrapped()));
    comp.set_output(Some(Stream::new_wrapped()));
    loop {
        let current = grid.get(&pos).copied().unwrap_or(0);
        log::debug!("current color: {}", current);
        comp.input().unwrap().borrow_mut().write(current as i64);
        let state = comp.execute();
        if state == ComputerState::Halted {
            break
        }
        let output = comp.output().unwrap();
        let paint = output.borrow_mut().read().expect("Expected a color to paint");
        let turn = output.borrow_mut().read().expect("Expected a direction to turn");
        grid.insert(pos, paint as u8);
        dir = match turn {
            0 => turn_left(dir),
            1 => turn_right(dir),
            _ => panic!("Unknown turn direction: {}", turn),
        };
        pos = move_n(pos, dir, 1);
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
    }
}

fn move_n((x, y): Point, dir: Direction, n: i32) -> Point {
    match dir {
        Direction::Up => (x, y + n),
        Direction::Left => (x - n, y),
        Direction::Down => (x, y - n),
        Direction::Right => (x + n, y),
    }
}

fn print_grid(grid: &HashMap<Point, u8>) {
    let minx = grid.keys().map(|&(x, _)| x).min().unwrap_or(0);
    let miny = grid.keys().map(|&(_, y)| y).min().unwrap_or(0);
    let maxx = grid.keys().map(|&(x, _)| x).max().unwrap_or(0);
    let maxy = grid.keys().map(|&(_, y)| y).max().unwrap_or(0);

    for y in (miny..maxy+1).rev() {
        let mut s = String::new();
        for x in minx..maxx+1 {
            let color = grid.get(&(x, y)).copied().unwrap_or(0);
            s.push(if color == 1 { '#' } else { ' ' })
        }
        println!("{}", s);
    }
}
