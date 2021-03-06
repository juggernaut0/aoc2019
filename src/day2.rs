use crate::intcode::*;

pub fn run1(input: Vec<String>) -> i64 {
    execute_program(parse_program(&input[0]), 12, 2)
}

pub fn run2(input: Vec<String>) -> i64 {
    let original = parse_program(&input[0]);
    let target = 19690720;
    for (n, v) in itertools::iproduct!(0..100, 0..100) {
        let output = execute_program(original.clone(), n, v);
        if output == target {
            return 100 * n + v
        }
    }
    panic!("Could not find target!");
}

fn execute_program(mut program: Program, n: i64, v: i64) -> i64 {
    program[1] = n;
    program[2] = v;
    execute_no_io(&mut program);
    program[0]
}
