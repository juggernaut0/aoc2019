use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use crate::intcode::ComputerState::{Halted, WaitingOnInput};
use std::ops::DerefMut;

pub type Program = Vec<i32>;

pub fn parse_program(s: &str) -> Program {
    s.split(',').map(|it| it.trim().parse().unwrap()).collect()
}

pub fn execute_no_io(program: &mut Program) {
    execute_streaming::<&mut Stream, &mut Stream>(program, 0, None, None);
}

pub fn execute(program: &mut Program, input: &mut dyn Iterator<Item=&i32>) -> Vec<i32> {
    let mut output = Stream::new();
    let (state, _) = execute_streaming(program, 0, Some(&mut Stream::from_iter(input)), Some(&mut output));
    if state == WaitingOnInput {
        panic!("Unexpected end of input");
    }
    output.into_vec()
}

fn execute_streaming<TIn, TOut>(program: &mut Program, init_ip: usize, mut input: Option<TIn>, mut output: Option<TOut>) -> (ComputerState, usize)
        where TIn: DerefMut<Target=Stream>,
              TOut: DerefMut<Target=Stream>,
{
    let mut ip: usize = init_ip;
    loop {
        let instr = program[ip];
        log::trace!("ip: {} instr: {}", ip, instr);
        let op = instr % 100;
        let pm1 = read_param_mode(instr, 1000);
        let pm2 = read_param_mode(instr, 10000);
        //let pm3 = read_param_mode(instr, 100000);
        match op {
            1 => {
                let a = read(program, program[ip + 1], pm1);
                let b = read(program, program[ip + 2], pm2);
                let dest = program[ip + 3] as usize;
                program[dest] = a + b;
                ip += 4;
            }
            2 => {
                let a = read(program, program[ip + 1], pm1);
                let b = read(program, program[ip + 2], pm2);
                let dest = program[ip + 3] as usize;
                program[dest] = a * b;
                ip += 4;
            }
            3 => {
                log::debug!("trying to read...");
                let inp = match input.as_mut().and_then(|s| (*s).read()) {
                    Some(x) => x,
                    None => return (WaitingOnInput, ip),
                };
                log::debug!("got an input: {}", inp);
                let dest = program[ip + 1] as usize;
                program[dest] = inp;
                ip += 2;
            }
            4 => {
                let a = read(program, program[ip + 1], pm1);
                if let Some(s) = output.as_mut() {
                    (*s).write(a);
                }
                ip += 2;
            }
            5 => {
                let a = read(program, program[ip + 1], pm1);
                let t = read(program, program[ip + 2], pm2);
                if a != 0 {
                    ip = t as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let a = read(program, program[ip + 1], pm1);
                let t = read(program, program[ip + 2], pm2);
                if a == 0 {
                    ip = t as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let a = read(program, program[ip + 1], pm1);
                let b = read(program, program[ip + 2], pm2);
                let dest = program[ip + 3] as usize;
                program[dest] = if a < b { 1 } else { 0 };
                ip += 4;
            }
            8 => {
                let a = read(program, program[ip + 1], pm1);
                let b = read(program, program[ip + 2], pm2);
                let dest = program[ip + 3] as usize;
                program[dest] = if a == b { 1 } else { 0 };
                ip += 4;
            }
            99 => return (Halted, ip),
            _ => panic!("Unrecognized opcode: {} @ ip {}", op, ip)
        }
    }
}

fn read_param_mode(instr: i32, place: i32) -> i32 {
    instr % place / (place / 10)
}

fn read(program: &mut Program, param: i32, mode: i32) -> i32 {
    match mode {
        0 => program[param as usize],
        1 => param,
        _ => panic!("Unsupported parameter mode: {}", mode)
    }
}

pub struct Computer {
    program: Program,
    ip: usize,
    input: Option<Rc<RefCell<Stream>>>,
    output: Option<Rc<RefCell<Stream>>>,
}

impl Computer {
    pub fn new(program: Program) -> Computer {
        Computer {
            program,
            ip: 0,
            input: None,
            output: None
        }
    }

    pub fn input(&self) -> Option<Rc<RefCell<Stream>>> {
        self.input.clone()
    }

    pub fn set_input(&mut self, input: Option<Rc<RefCell<Stream>>>) {
        self.input = input;
    }

    pub fn output(&self) -> Option<Rc<RefCell<Stream>>> {
        self.output.clone()
    }

    pub fn set_output(&mut self, output: Option<Rc<RefCell<Stream>>>) {
        self.output = output;
    }

    pub fn execute(&mut self) -> ComputerState {
        let input = self.input.as_ref().map(|it| it.borrow_mut());
        let output = self.output.as_ref().map(|it| it.borrow_mut());
        let (state, ip) = execute_streaming(&mut self.program, self.ip, input, output);
        self.ip = ip;
        state
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ComputerState {
    Halted,
    WaitingOnInput
}

pub struct Stream {
    store: VecDeque<i32>
}

impl Stream {
    pub fn new() -> Stream {
        Stream {
            store: VecDeque::new()
        }
    }

    fn from_iter(iter: &mut dyn Iterator<Item=&i32>) -> Stream {
        Stream {
            store: iter.copied().collect()
        }
    }

    fn into_vec(self) -> Vec<i32> {
        self.store.into()
    }

    pub fn read(&mut self) -> Option<i32> {
        self.store.pop_front()
    }

    pub fn write(&mut self, value: i32) {
        self.store.push_back(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let mut program = vec![1,1,1,4,99,5,6,0,99];
        execute_no_io(&mut program);
        assert_eq!(vec![30,1,1,4,2,5,6,0,99], program);
    }

    #[test]
    fn param_modes() {
        assert_eq!(0, read_param_mode(23, 1000));
        assert_eq!(0, read_param_mode(23, 10000));
        assert_eq!(1, read_param_mode(123, 1000));
        assert_eq!(0, read_param_mode(123, 10000));
        assert_eq!(0, read_param_mode(1023, 1000));
        assert_eq!(1, read_param_mode(1023, 10000));
        assert_eq!(1, read_param_mode(1123, 1000));
        assert_eq!(1, read_param_mode(1123, 10000));
    }

    #[test]
    fn jumps() {
        let program = parse_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        assert_eq!(vec![0], execute(&mut program.clone(), &mut [0].iter()));
        assert_eq!(vec![1], execute(&mut program.clone(), &mut [1].iter()));
        assert_eq!(vec![1], execute(&mut program.clone(), &mut [2].iter()));
    }

    #[test]
    fn jumps2() {
        let program = parse_program(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        assert_eq!(vec![999], execute(&mut program.clone(), &mut [2].iter()));
        assert_eq!(vec![1000], execute(&mut program.clone(), &mut [8].iter()));
        assert_eq!(vec![1001], execute(&mut program.clone(), &mut [42].iter()));
    }
}
