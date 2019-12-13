use crate::intcode::*;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

pub fn run1(input: Vec<String>) -> usize {
    let mut comp = Computer::new(parse_program(&input[0]));
    comp.set_output(Some(Stream::new_wrapped()));
    comp.execute();
    let output = comp.output().unwrap().borrow_mut().read_all();
    log::debug!("output.len: {}", output.len());
    let mut screen = HashMap::new();
    update_screen(&mut screen, output);
    log::debug!("{:#?}", screen);
    screen.values().filter(|&&it| it == 2).count()
}

pub fn run2(input: Vec<String>) {
    let mut program = parse_program(&input[0]);
    program[0] = 2;
    let mut comp = Computer::new(program);
    comp.set_input(Some(Stream::new_wrapped()));
    comp.set_output(Some(Stream::new_wrapped()));
    let mut screen = HashMap::new();
    let mut inp = String::new();
    let mut last_ball_pos: Option<i64> = None;
    while ComputerState::WaitingOnInput == comp.execute() {
        let output = comp.output().unwrap().borrow_mut().read_all();
        update_screen(&mut screen, output);

        let ball_pos = find_ball(&screen);
        let paddle = find_paddle(&screen);
        let i = if ball_pos == paddle {
            log::debug!("input: 0");
            0
        } else if let Some(last_pos) = last_ball_pos {
            let res = ball_pos - last_pos;
            log::debug!("input: {}", res);
            res
        } else {
            log::debug!("initial input: -1");
            -1
        };
        last_ball_pos = Some(ball_pos);

        // Comment to skip watching it play
        print_screen(&screen);
        inp.clear();
        print!(">>> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut inp).unwrap();

        comp.input().unwrap().borrow_mut().write(i);
    }
    let output = comp.output().unwrap().borrow_mut().read_all();
    update_screen(&mut screen, output);
    print_screen(&screen);
}

fn update_screen(screen: &mut HashMap<(Value, Value), Value>, output: Vec<Value>) {
    output.chunks(3)
        .map(|it| (it[0], it[1], it[2]))
        .for_each(|(x, y, tile)| {
            screen.insert((x, y), tile);
        });
}

fn print_screen(screen: &HashMap<(Value, Value), Value>) {
    let score = screen.get(&(-1, 0)).copied().unwrap_or(0);
    println!("Score: {}", score);
    let width = screen.keys().map(|&(x, _)| x).max().unwrap_or(0) + 1;
    let height = screen.keys().map(|&(_, y)| y).max().unwrap_or(0) + 1;
    for y in 0..height {
        let mut s = String::new();
        for x in 0..width {
            let tile = screen.get(&(x, y)).copied().unwrap_or(0);
            let c = match tile {
                0 => ' ',
                1 => '█',
                2 => '#',
                3 => '=',
                4 => '*',
                _ => panic!("Unknown tile: {}", tile)
            };
            s.push(c)
        }
        println!("{}", s);
    }
}

fn find_ball(screen: &HashMap<(Value, Value), Value>) -> i64 {
    screen.iter().find(|&(_, &tile)| tile == 4).map(|(&(x, _), _)| x).unwrap()
}

fn find_paddle(screen: &HashMap<(Value, Value), Value>) -> i64 {
    screen.iter().find(|&(_, &tile)| tile == 3).map(|(&(x, _), _)| x).unwrap()
}
