use crate::intcode::*;
use std::rc::Rc;
use std::io::{stdout, stdin, Write};
use std::collections::{HashMap, HashSet};

pub fn run1(input: Vec<String>) -> usize {
    let mut comp = Computer::new(parse_program(&input[0]));
    let input = Stream::new_wrapped();
    let output = Stream::new_wrapped();
    comp.set_input(Some(Rc::clone(&input)));
    comp.set_output(Some(Rc::clone(&output)));
    let mut human_input = String::new();
    let mut map = HashMap::new();
    let mut robot = (0, 0);
    let mut last_input = None;
    let mut agent = Agent::new();
    loop {
        match comp.execute() {
            ComputerState::Halted => panic!("unexpected halt"),
            ComputerState::WaitingOnInput => {
                if let Some(signal) = output.borrow_mut().read() {
                    match signal {
                        0 => {
                            agent.inform_wall();
                            map.insert(new_pos(robot, last_input.unwrap()), Tile::Wall);
                        }
                        1 => {
                            robot = new_pos(robot, last_input.unwrap());
                            agent.inform_move();
                            map.insert(robot, Tile::Open);
                        }
                        2 => {
                            agent.inform_move();
                            log::info!("goal pos: {:?}", new_pos(robot, last_input.unwrap()));
                            break agent.path_length()
                        },
                        _ => panic!("Unexpected output signal: {}", signal)
                    }
                }
                print_screen(&map, robot);
                prompt(&mut human_input);
                let dir = agent.decide_move(&map, robot);
                input.borrow_mut().write(dir.to_input());
                last_input = Some(dir);
            },
        }
    }
}

pub fn run2(input: Vec<String>) -> u32 {
    let mut comp = Computer::new(parse_program(&input[0]));
    let input = Stream::new_wrapped();
    let output = Stream::new_wrapped();
    comp.set_input(Some(Rc::clone(&input)));
    comp.set_output(Some(Rc::clone(&output)));
    let mut map = HashMap::new();
    let mut robot = (0, 0);
    let mut last_input = None;
    let mut agent = Agent::new();
    let mut has_moved = false;
    let mut oxygen = (1000, 1000);

    while agent.path_length() > 0 || !has_moved {
        log::trace!("loop");
        match comp.execute() {
            ComputerState::Halted => panic!("unexpected halt"),
            ComputerState::WaitingOnInput => {
                if let Some(signal) = output.borrow_mut().read() {
                    match signal {
                        0 => {
                            agent.inform_wall();
                            map.insert(new_pos(robot, last_input.unwrap()), Tile::Wall);
                        }
                        1 => {
                            robot = new_pos(robot, last_input.unwrap());
                            agent.inform_move();
                            map.insert(robot, Tile::Open);
                            has_moved = true;
                        }
                        2 => {
                            robot = new_pos(robot, last_input.unwrap());
                            agent.inform_move();
                            map.insert(robot, Tile::Oxygen);
                            has_moved = true;
                            oxygen = robot;
                        },
                        _ => panic!("Unexpected output signal: {}", signal)
                    }
                }
                let dir = agent.decide_move(&map, robot);
                input.borrow_mut().write(dir.to_input());
                last_input = Some(dir);
            },
        }
    }
    
    let mut tips = HashSet::new();
    tips.insert(oxygen);
    let mut t = 0;
    while map.values().any(|&it| it == Tile::Open) {
        tips = spread_oxygen(tips, &mut map);
        t += 1;
    }
    t
}

fn prompt(inp: &mut String) {
    inp.clear();
    print!(">>> ");
    stdout().flush().unwrap();
    stdin().read_line(inp).unwrap();
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Open,
    Oxygen,
}

#[derive(Copy, Clone)]
enum Direction {
    North, South, West, East
}

impl Direction {
    fn to_input(&self) -> Value {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

type Map = HashMap<(Value, Value), Tile>;

fn print_screen(screen: &Map, robot: (Value, Value)) {
    let width_range = {
        let xs = screen.keys().map(|&(x, _)| x);
        let min = xs.clone().min().unwrap_or(0);
        let max = xs.max().unwrap_or(0) + 1;
        min..max
    };
    let height_range = {
        let ys = screen.keys().map(|&(_, y)| y);
        let min = ys.clone().min().unwrap_or(0);
        let max = ys.max().unwrap_or(0) + 1;
        min..max
    };
    for y in height_range.rev() {
        let mut s = String::new();
        for x in width_range.clone() {
            let c = if (x, y) == robot {
                'D'
            } else {
                match screen.get(&(x, y)).copied() {
                    None => ' ',
                    Some(Tile::Open) => '.',
                    Some(Tile::Wall) => '█',
                    Some(Tile::Oxygen) => 'O'
                }
            };
            s.push(c);
            s.push(c);
        }
        println!("{}", s);
    }
}

fn new_pos(pos: (Value, Value), dir: Direction) -> (Value, Value) {
    match dir {
        Direction::North => (pos.0, pos.1 + 1),
        Direction::South => (pos.0, pos.1 - 1),
        Direction::West => (pos.0 - 1, pos.1),
        Direction::East => (pos.0 + 1, pos.1),
    }
}

struct Agent {
    stack: Vec<Direction>,
    pending: Option<Direction>,
}

impl Agent {
    fn new() -> Agent {
        Agent {
            stack: Vec::new(),
            pending: None,
        }
    }

    fn decide_move(&mut self, map: &Map, pos: (Value, Value)) -> Direction {
        let dir = if map.get(&new_pos(pos, Direction::North)).is_none() {
            Direction::North
        } else if map.get(&new_pos(pos, Direction::West)).is_none() {
            Direction::West
        } else if map.get(&new_pos(pos, Direction::South)).is_none() {
            Direction::South
        } else if map.get(&new_pos(pos, Direction::East)).is_none() {
            Direction::East
        } else {
            return self.stack.pop().unwrap().opposite()
        };
        self.pending = Some(dir);
        dir
    }

    fn inform_move(&mut self) {
        if let Some(dir) = self.pending.take() {
            self.stack.push(dir)
        }
    }

    fn inform_wall(&mut self) {
        self.pending = None;
    }

    fn path_length(&self) -> usize {
        self.stack.len()
    }
}

fn spread_oxygen(tips: HashSet<(Value, Value)>, map: &mut Map) -> HashSet<(Value, Value)> {
    let mut new_tips = HashSet::new();
    for &t in &tips {
        for &dir in [Direction::North, Direction::East, Direction::South, Direction::West].iter() {
            let new_pos = new_pos(t, dir);
            if let Some(Tile::Open) = map.get(&new_pos) {
                new_tips.insert(new_pos);
            }
        }
    }
    for &t in &new_tips {
        map.insert(t, Tile::Oxygen);
    }
    new_tips
}
