use crate::intcode;

pub fn run1(input: Vec<String>) -> Vec<i64> {
    let mut program = intcode::parse_program(&input[0]);
    intcode::execute(&mut program, &mut [1].iter())
}

pub fn run2(input: Vec<String>) -> Vec<i64> {
    let mut program = intcode::parse_program(&input[0]);
    intcode::execute(&mut program, &mut [2].iter())
}
