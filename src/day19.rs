use crate::intcode::*;
use std::rc::Rc;

pub fn run1(input: Vec<String>) -> i64 {
    let program = parse_program(&input[0]);
    let mut sum = 0;
    for x in 0..50 {
        for y in 0..50 {
            sum += test(program.clone(), x, y);
        }
    }
    sum
}

pub fn run2(input: Vec<String>) -> i64 {
    let program = parse_program(&input[0]);

    let mut x = 700;
    //let mut last_y = 0;
    let (tlx, tly) = loop {
        let mut y = 900;
        loop {
            let output = test(program.clone(), x, y);
            if output == 1 {
                break;
            }
            y += 1;
        }
        log::info!("tr ({}, {}) -> 1", x, y);
        let output = test(program.clone(), x - 99, y + 99);
        log::info!("bl ({}, {}) -> {}", x - 99, y + 99, output);
        if validate(&program, x - 99, y) {
            break ((x - 99), y);
        }
        x += 1;
        //last_y = y;
    };

    for y in 1050..1200 {
        let mut s = String::new();
        for x in 600..800 {
            if (tlx..tlx+100).contains(&x) && (tly..tly+100).contains(&y) {
                if test(program.clone(), x, y) == 1 {
                    s.push('O');
                } else {
                    s.push('X')
                }
            } else if test(program.clone(), x, y) == 1 {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        println!("{}", s);
    }

    return tlx * 10000 + tly;
}

fn test(program: Program, x: Value, y: Value) -> Value {
    let mut comp = Computer::new(program);
    let input = Stream::new_wrapped();
    let output = Stream::new_wrapped();
    comp.set_input(Some(Rc::clone(&input)));
    comp.set_output(Some(Rc::clone(&output)));
    input.borrow_mut().write_all(&[x, y]);
    comp.execute();
    let value = output.borrow_mut().read().unwrap();
    value
}

fn validate(program: &Program, tlx: Value, tly: Value) -> bool {
    for x in tlx..tlx+100 {
        for y in tly..tly+100 {
            if test(program.clone(), x, y) == 0 {
                log::warn!("Outside beam: ({}, {})", x, y);
                return false
            }
        }
    }
    return true
}
