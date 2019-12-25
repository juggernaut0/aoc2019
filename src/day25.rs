use crate::intcode::*;
use std::rc::Rc;
use std::io::stdin;
use std::collections::HashSet;
use itertools::Itertools;
use std::hash::Hash;

pub fn run1(input: Vec<String>) {
    let mut comp = Computer::new(parse_program(&input[0]));
    let in_st = Stream::new_wrapped();
    let out_st = Stream::new_wrapped();
    comp.set_input(Some(Rc::clone(&in_st)));
    comp.set_output(Some(Rc::clone(&out_st)));

    let take_everything = "\
west
south
take festive hat
east
take whirled peas
west
south
take sand
north
north
west
north
take space heater
south
east
east
south
take weather machine
north
east
take mug
east
south
east
south
take easter egg
north
west
west
south
west
take shell
south
drop easter egg
drop mug
drop sand
drop weather machine
drop festive hat
drop shell
drop whirled peas
drop space heater
";
    in_st.borrow_mut().write_ascii(take_everything);

    comp.execute();
    println!("{}", out_st.borrow_mut().read_ascii());

    let ps = power_set(vec![
        "easter egg",
        "mug",
        "sand",
        "weather machine",
        "festive hat",
        "shell",
        "whirled peas",
        "space heater",
    ]);

    for items in ps {
        let mut s = String::new();
        for item in &items {
            s.push_str(&format!("take {}\n", item));
        }
        s.push_str("south\n");
        in_st.borrow_mut().write_ascii(&s);
        comp.execute();
        println!("{}", out_st.borrow_mut().read_ascii());
        println!("{:?}", items);
        prompt(&mut String::new()); // ignored

        s.clear();
        for item in &items {
            s.push_str(&format!("drop {}\n", item));
        }
        in_st.borrow_mut().write_ascii(&s);
        comp.execute();
    }

    let mut buf = String::new();
    while let ComputerState::WaitingOnInput = comp.execute() {
        println!("{}", out_st.borrow_mut().read_ascii());
        prompt(&mut buf);
        in_st.borrow_mut().write_ascii(&buf);
    }
    println!("{}", out_st.borrow_mut().read_ascii());
}

pub fn run2(_input: Vec<String>) -> &'static str {
    "Merry Christmas!"
}

fn prompt(inp: &mut String) {
    inp.clear();
    stdin().read_line(inp).unwrap();
}

fn power_set<T : Clone + Eq + Hash>(v: Vec<T>) -> Vec<HashSet<T>> {
    let mut res = Vec::new();
    for k in 0..=v.len() {
        for s in v.iter().combinations(k) {
            res.push(s.into_iter().cloned().collect())
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn power_set_test() {
        let ps = power_set((0..4).collect());
        println!("{:?}", ps);
    }
}
