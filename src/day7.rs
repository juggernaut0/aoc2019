use crate::intcode;
use itertools::Itertools;
use std::rc::Rc;
use std::cell::RefCell;
use crate::intcode::ComputerState;

pub fn run1(input: Vec<String>) -> i32 {
    let program = intcode::parse_program(&input[0]);
    vec![0, 1, 2, 3, 4]
        .into_iter()
        .permutations(5)
        .map(|phases| run_amps_once(&program, &phases[..]))
        .max()
        .unwrap()
}

pub fn run2(input: Vec<String>) -> i32 {
    let program = intcode::parse_program(&input[0]);
    vec![5, 6, 7, 8, 9]
        .into_iter()
        .permutations(5)
        .map(|phases| run_amps_looped(&program, &phases[..]))
        .max()
        .unwrap()
}

fn run_amps_once(program: &intcode::Program, phases: &[i32]) -> i32 {
    let mut signal = 0;
    for i in 0..5 {
        let input = [phases[i], signal];
        let output = intcode::execute(&mut program.clone(), &mut input.iter());
        signal = output[0];
    }
    signal
}

fn run_amps_looped(program: &intcode::Program, phases: &[i32]) -> i32 {
    let mut computers = vec![
        intcode::Computer::new(program.clone()),
        intcode::Computer::new(program.clone()),
        intcode::Computer::new(program.clone()),
        intcode::Computer::new(program.clone()),
        intcode::Computer::new(program.clone()),
    ];

    for i in 0..5 {
        let mut s = intcode::Stream::new();
        s.write(phases[i]);
        if i == 0 {
            s.write(0);
        }
        computers[i].set_input(Some(Rc::new(RefCell::new(s))));
    }

    for i in 0..5 {
        let o = (i + 1) % 5;
        let s = computers[o].input();
        computers[i].set_output(s);
    }

    let mut running = 4;
    let mut limit = 1000;
    loop {
        log::debug!("Starting execute of amp {}", running);
        let state = computers[running].execute();
        log::debug!("Amp {} suspended with state: {:?}", running, state);
        if state == ComputerState::Halted && running == 4 {
            break
        }

        running = (running + 4) % 5;

        limit -= 1;
        if limit == 0 {
            panic!("Hit the limit")
        }
    }
    computers[4].output().unwrap().borrow_mut().read().expect("Expect a final value")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let program = intcode::parse_program(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
             27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
        let output = run_amps_looped(&program, &[9,8,7,6,5]);
        assert_eq!(139629729, output);
    }
}
