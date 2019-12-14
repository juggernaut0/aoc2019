use std::cmp::min;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::fmt::Debug;

pub fn run1(input: Vec<Reaction>) -> u64 {
    let reactions_by_output = input.into_iter().map(|it| (it.output.0.clone(), it)).collect();
    calculate_ore(1, &reactions_by_output)
}

pub fn run2(input: Vec<Reaction>) -> u64 {
    let reactions_by_output = input.into_iter().map(|it| (it.output.0.clone(), it)).collect();
    binary_search(2500000, 5000000, |n| {
        log::info!("n = {}", n);
        let ore = calculate_ore(n, &reactions_by_output);
        log::info!("ore = {}", ore);
        ore <= 1_000_000_000_000
    })
}

#[derive(Debug)]
pub struct Reaction {
    inputs: HashMap<String, u64>,
    output: (String, u64)
}

impl FromStr for Reaction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("=>").map(|it| it.trim()).collect();
        let inputs = parts[0].split(',')
            .map(|it| {
                let mut p = it.trim().split(' ');
                let count = p.next().unwrap().trim().parse().unwrap();
                let chem = p.next().unwrap().trim().to_string();
                (chem, count)
            })
            .collect();
        let output = {
            let mut p = parts[1].split(' ');
            let count = p.next().unwrap().trim().parse()?;
            let chem = p.next().unwrap().trim().to_string();
            (chem, count)
        };
        Ok(Reaction { inputs, output })
    }
}

fn calculate_ore(fuel_needed: u64, reactions_by_output: &HashMap<String, Reaction>) -> u64 {
    let mut ore = 0;
    let mut needs = Vec::new();
    let mut spare = HashMap::new();
    needs.push(("FUEL".to_string(), fuel_needed));
    while let Some((chem, mut count)) = needs.pop() {
        log::debug!("current = ({}, {})", chem, count);
        log::debug!("needs = {:?}", needs);
        log::debug!("spare = {:?}", spare);

        if let Some(&have) = spare.get(&chem) {
            let amt = min(have, count);
            count -= amt;
            let new_spare = have - amt;
            spare.insert(chem.clone(), new_spare);
        }

        let reaction = match reactions_by_output.get(&chem) {
            Some(r) => r,
            None => {
                ore += count;
                continue;
            }
        };
        log::debug!("reaction = {:#?}", reaction);
        let output_amt = reaction.output.1;

        let times = if count % output_amt == 0 {
            count / output_amt
        } else {
            (count / output_amt) + 1
        };
        let extra = output_amt * times - count;

        log::debug!("times = {} extra = {}", times, extra);

        let mut inputs: HashMap<_, _> = reaction.inputs.iter().map(|(c, &a)| (c.clone(), a * times)).collect();
        for (inp_chem, req) in inputs.iter_mut() {
            if let Some(&have) = spare.get(inp_chem) {
                let amt = min(have, *req);
                *req -= amt;
                let new_spare = have - amt;
                spare.insert(inp_chem.clone(), new_spare);
            }
        }

        for need in inputs {
            needs.push(need)
        }

        if extra > 0 {
            let new_spare = spare.get(&chem).copied().unwrap_or(0) + extra;
            spare.insert(chem, new_spare);
        }
    }
    ore
}

fn binary_search(mut min: u64, mut max: u64, test: impl Fn(u64) -> bool) -> u64 {
    loop {
        let mid = (min + max) / 2;
        if test(mid) {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
        if min == max {
            return min
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_input;

    #[test]
    fn pt1ex1() {
        let input = parse_input("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");

        assert_eq!(13312, run1(input));
    }

    #[test]
    fn pt1ex2() {
        let input = parse_input("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF");

        assert_eq!(180697, run1(input));
    }
}
