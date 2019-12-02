pub fn run1(input: Vec<String>) -> u32 {
    execute_program(parse_program(&input[0]), 12, 2)
}

pub fn run2(input: Vec<String>) -> u32 {
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

fn parse_program(s: &str) -> Vec<u32> {
    s.split(',').map(|it| it.parse().unwrap()).collect()
}

fn execute_program(mut program: Vec<u32>, n: u32, v: u32) -> u32 {
    program[1] = n;
    program[2] = v;
    execute(&mut program);
    program[0]
}

fn execute(program: &mut Vec<u32>) {
    let mut ip: usize = 0;
    loop {
        log::debug!("{:?}", program);
        let op = program[ip];
        match op {
            1 => {
                let a = program[program[ip + 1] as usize];
                let b = program[program[ip + 2] as usize];
                let dest = program[ip + 3] as usize;
                program[dest] = a + b;
            }
            2 => {
                let a = program[program[ip + 1] as usize];
                let b = program[program[ip + 2] as usize];
                let dest = program[ip + 3] as usize;
                program[dest] = a * b;
            }
            99 => return,
            _ => panic!("Unrecognized opcode: {}", op)
        }
        ip += 4
    }
}

#[cfg(test)]
mod test {
    use crate::day2::execute;
    use log::Level;

    #[test]
    fn simple() {
        simple_logger::init_with_level(Level::Debug).unwrap();
        let mut program = vec![1,1,1,4,99,5,6,0,99];
        execute(&mut program);
        assert_eq!(vec![30,1,1,4,2,5,6,0,99], program);
    }
}
