use crate::intcode::*;
use std::rc::Rc;

pub fn run1(input: Vec<String>) -> Value {
    let js = "\
NOT A T
OR T J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";
    execute_js(input, js)
}

pub fn run2(input: Vec<String>) -> Value {
    let js = "\
NOT A T
OR T J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT H T
NOT T T
OR E T
AND T J
RUN
";
    execute_js(input, js)
}

fn execute_js(prog: Vec<String>, js: &str) -> Value {
    let mut comp = Computer::new(parse_program(&prog[0]));
    let input = Stream::new_wrapped();
    let output = Stream::new_wrapped();
    comp.set_input(Some(Rc::clone(&input)));
    comp.set_output(Some(Rc::clone(&output)));

    assert_eq!(ComputerState::WaitingOnInput, comp.execute());
    output.borrow_mut().read_all();

    input.borrow_mut().write_all(&to_input(js));
    comp.execute();

    let out = output.borrow_mut().read_all();
    let last = out.last().copied().unwrap();
    if last > 127 {
        last
    } else {
        println!("{}", to_string(&out));
        panic!("Oopsy woopsy");
    }
}

fn to_input(s: &str) -> Vec<i64> {
    s.chars().map(|c| c as u32 as i64).collect()
}

fn to_string(vs: &[Value]) -> String {
    vs.iter().map(|&v| v as u8 as char).collect()
}
