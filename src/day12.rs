use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;

pub fn run1(input: Vec<Moon>) -> u32 {
    let moons = wrap_moons(input);

    for _ in 0..1000 {
        step(&moons);
    }

    moons.iter()
        .map(|moon| moon.borrow().energy())
        .sum()
}

pub fn run2(input: Vec<Moon>) -> u64 {
    let moons = wrap_moons(input);
    let mut x_states = HashSet::new();
    let mut y_states = HashSet::new();
    let mut z_states = HashSet::new();
    let mut steps = 0;
    let mut x_steps = None;
    let mut y_steps = None;
    let mut z_steps = None;
    loop {
        let mut x_hasher = DefaultHasher::new();
        let mut y_hasher = DefaultHasher::new();
        let mut z_hasher = DefaultHasher::new();
        moons.iter().for_each(|it| {
            it.borrow().pos.0.hash(&mut x_hasher);
            it.borrow().vel.0.hash(&mut x_hasher);
            it.borrow().pos.1.hash(&mut y_hasher);
            it.borrow().vel.1.hash(&mut y_hasher);
            it.borrow().pos.2.hash(&mut z_hasher);
            it.borrow().vel.2.hash(&mut z_hasher);
        });
        if !x_states.insert(x_hasher.finish()) && x_steps.is_none() {
            x_steps = Some(steps)
        }
        if !y_states.insert(y_hasher.finish()) && y_steps.is_none() {
            y_steps = Some(steps)
        }
        if !z_states.insert(z_hasher.finish()) && z_steps.is_none() {
            z_steps = Some(steps)
        }
        if x_steps.is_some() && y_steps.is_some() && z_steps.is_some() {
            break
        }
        if steps % 10000 == 0 {
            log::info!("steps: {} x_steps: {:?} y_steps: {:?} z_steps: {:?}", steps, x_steps, y_steps, z_steps)
        }
        step(&moons);
        steps += 1;
    }
    log::info!("steps: {} x_steps: {:?} y_steps: {:?} z_steps: {:?}", steps, x_steps, y_steps, z_steps);
    lcm3(x_steps.unwrap(), y_steps.unwrap(), z_steps.unwrap())
}

type Vector = (i32, i32, i32);
#[derive(Clone, Hash)]
pub struct Moon {
    pos: Vector,
    vel: Vector,
}

impl FromStr for Moon {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let comps: Result<Vec<i32>, _> = s[1..s.len()-1].split(", ").map(|it| it[2..it.len()].parse()).collect();
        let comps = comps?;
        let pos = (comps[0], comps[1], comps[2]);
        Ok(Moon { pos, vel: (0, 0, 0) })
    }
}

impl Moon {
    fn energy(&self) -> u32 {
        let pot = self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs();
        let kin = self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs();
        (pot * kin) as u32
    }
}

fn wrap_moons(input: Vec<Moon>) -> Vec<RefCell<Moon>> {
    input.into_iter().map(|it| RefCell::new(it)).collect()
}

fn step(moons: &Vec<RefCell<Moon>>) {
    (0..4).permutations(2)
        .map(|it| (it[0], it[1]))
        .filter(|&(ai, bi)| ai < bi)
        .for_each(|(ai, bi)| {
            let a = &moons[ai];
            let b = &moons[bi];
            gravitate(&mut a.borrow_mut(), &mut b.borrow_mut());
        });
    moons.iter().for_each(|moon| velocitate(&mut moon.borrow_mut()));
}

fn gravitate(a: &mut Moon, b: &mut Moon) {
    let (ax, ay, az) = a.pos;
    let (bx, by, bz) = b.pos;

    if ax > bx {
        a.vel.0 -= 1;
        b.vel.0 += 1;
    } else if ax < bx {
        a.vel.0 += 1;
        b.vel.0 -= 1;
    }

    if ay > by {
        a.vel.1 -= 1;
        b.vel.1 += 1;
    } else if ay < by {
        a.vel.1 += 1;
        b.vel.1 -= 1;
    }

    if az > bz {
        a.vel.2 -= 1;
        b.vel.2 += 1;
    } else if az < bz {
        a.vel.2 += 1;
        b.vel.2 -= 1;
    }
}

fn velocitate(moon: &mut Moon) {
    moon.pos = (moon.vel.0 + moon.pos.0, moon.vel.1 + moon.pos.1, moon.vel.2 + moon.pos.2);
}

fn gcd(a: u64, b: u64) -> u64 {
    if a > b {
        gcd(a - b, b)
    } else if a < b {
        gcd(a, b - a)
    } else {
        a
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    let gcd = gcd(a, b);
    (a / gcd) * (b / gcd)
}

fn lcm3(a: u64, b: u64, c: u64) -> u64 {
    lcm(lcm(a, b), c)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_input;

    #[test]
    fn energy() {
        assert_eq!(36, Moon { pos: (2, 1, -3), vel: (-3, -2, 1) }.energy());
    }

    #[test]
    fn gravitate() {
        let input = parse_input("<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>");
        let moons = wrap_moons(input);
        step(&moons);
        assert_eq!((2, -1, 1), moons[0].borrow().pos);
        assert_eq!((3, -1, -1), moons[0].borrow().vel);
    }
}
