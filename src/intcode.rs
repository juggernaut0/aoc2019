use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use crate::intcode::ComputerState::{Halted, WaitingOnInput};

pub type Value = i64;
pub type Program = Vec<Value>;

pub fn parse_program(s: &str) -> Program {
    s.split(',').map(|it| it.trim().parse().unwrap()).collect()
}

pub fn execute_no_io(program: &mut Program) {
    let mut comp = Computer::new(program.clone());
    let state = comp.execute();
    if state == WaitingOnInput {
        panic!("Unexpected end of input");
    }
    *program = comp.program
}

pub fn execute(program: &mut Program, input: &mut dyn Iterator<Item=&Value>) -> Vec<Value> {
    let mut comp = Computer::new(program.clone());
    comp.set_input(Some(Rc::new(RefCell::new(Stream::from_iter(input)))));
    comp.set_output(Some(Rc::new(RefCell::new(Stream::new()))));
    let state = comp.execute();
    if state == WaitingOnInput {
        panic!("Unexpected end of input");
    }
    let ob = comp.output().unwrap();
    let r = VecDeque::clone(&ob.borrow().store).into();
    r
}

fn read_param_mode(instr: Value, place: i32) -> i32 {
    (instr as i32) % place / (place / 10)
}

pub struct Computer {
    program: Program,
    ip: usize,
    rel_base: usize,
    input: Option<Rc<RefCell<Stream>>>,
    output: Option<Rc<RefCell<Stream>>>,
}

impl Computer {
    pub fn new(program: Program) -> Computer {
        Computer {
            program,
            ip: 0,
            rel_base: 0,
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
        let mut ip: usize = self.ip;
        loop {
            let instr = self.program[ip];
            log::trace!("ip: {} instr: {}", ip, instr);
            let op = instr % 100;
            let pm1 = read_param_mode(instr, 1000);
            let pm2 = read_param_mode(instr, 10000);
            let pm3 = read_param_mode(instr, 100000);
            match op {
                1 => {
                    let a = self.read(self.program[ip + 1], pm1);
                    let b = self.read(self.program[ip + 2], pm2);
                    self.write(self.program[ip + 3], pm3, a + b);
                    ip += 4;
                }
                2 => {
                    let a = self.read(self.program[ip + 1], pm1);
                    let b = self.read(self.program[ip + 2], pm2);
                    self.write(self.program[ip + 3], pm3, a * b);
                    ip += 4;
                }
                3 => {
                    log::debug!("trying to read...");
                    let mut input = self.input.as_ref().map(|it| it.borrow_mut());
                    let inp = match input.as_mut().and_then(|s| (*s).read()) {
                        Some(x) => x,
                        None => {
                            self.ip = ip;
                            return WaitingOnInput
                        },
                    };
                    log::debug!("got an input: {}", inp);
                    drop(input);
                    self.write(self.program[ip + 1], pm1, inp);
                    ip += 2;
                }
                4 => {
                    let a = self.read(self.program[ip + 1], pm1);
                    let mut output = self.output.as_ref().map(|it| it.borrow_mut());
                    if let Some(s) = output.as_mut() {
                        (*s).write(a);
                    }
                    ip += 2;
                }
                5 => {
                    let a = self.read(self.program[ip + 1], pm1);
                    let t = self.read(self.program[ip + 2], pm2);
                    if a != 0 {
                        ip = t as usize;
                    } else {
                        ip += 3;
                    }
                }
                6 => {
                    let a = self.read(self.program[ip + 1], pm1);
                    let t = self.read(self.program[ip + 2], pm2);
                    if a == 0 {
                        ip = t as usize;
                    } else {
                        ip += 3;
                    }
                }
                7 => {
                    let a = self.read(self.program[ip + 1], pm1);
                    let b = self.read(self.program[ip + 2], pm2);
                    self.write(self.program[ip + 3], pm3, if a < b { 1 } else { 0 });
                    ip += 4;
                }
                8 => {
                    let a = self.read(self.program[ip + 1], pm1);
                    let b = self.read(self.program[ip + 2], pm2);
                    self.write(self.program[ip + 3], pm3, if a == b { 1 } else { 0 });
                    ip += 4;
                }
                9 => {
                    let d = self.read(self.program[ip + 1], pm1);
                    self.rel_base = (self.rel_base as i64 + d) as usize;
                    ip += 2;
                }
                99 => {
                    self.ip = ip;
                    return Halted
                },
                _ => panic!("Unrecognized opcode: {} @ ip {}", op, ip)
            }
        }
    }

    fn read(&self, param: Value, mode: i32) -> Value {
        match mode {
            0 => self.program.get(param as usize).copied().unwrap_or(0),
            1 => param,
            2 => self.program.get((param + (self.rel_base as Value)) as usize).copied().unwrap_or(0),
            _ => panic!("Unsupported parameter mode: {}", mode)
        }
    }

    fn write(&mut self, param: Value, mode: i32, value: Value) {
        let addr = match mode {
            0 => param as usize,
            2 => (param + (self.rel_base as i64)) as usize,
            _ => panic!("Unsupported parameter mode: {}", mode)
        };
        if addr >= self.program.len() {
            self.program.resize(addr + 1, 0)
        }
        self.program[addr] = value;
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ComputerState {
    Halted,
    WaitingOnInput
}

pub struct Stream {
    store: VecDeque<Value>
}

impl Stream {
    pub fn new() -> Stream {
        Stream {
            store: VecDeque::new()
        }
    }

    pub fn new_wrapped() -> Rc<RefCell<Stream>> {
        Rc::new(RefCell::new(Stream::new()))
    }

    fn from_iter(iter: &mut dyn Iterator<Item=&Value>) -> Stream {
        Stream {
            store: iter.copied().collect()
        }
    }

    pub fn read(&mut self) -> Option<Value> {
        self.store.pop_front()
    }

    pub fn write(&mut self, value: Value) {
        self.store.push_back(value)
    }

    pub fn read_all(&mut self) -> Vec<Value> {
        let mut res = Vec::new();
        while let Some(v) = self.read() {
            res.push(v)
        }
        res
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

    #[test]
    fn large() {
        let mut program = parse_program("104,1125899906842624,99");
        assert_eq!(vec![1125899906842624], execute(&mut program, &mut[].iter()))
    }
}
