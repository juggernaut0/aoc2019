use std::str::FromStr;
use std::collections::{HashMap, HashSet};

pub fn run1(input: Vec<Orbit>) -> u32 {
    let orbits = to_orbits(input);
    orbits
        .keys()
        .map(|k| count_orbits(&orbits, k, "COM"))
        .sum()
}

pub fn run2(input: Vec<Orbit>) -> u32 {
    let orbits = to_orbits(input);
    let target = nearest_common_parent(&orbits, "YOU", "SAN");
    count_orbits(&orbits, "YOU", target) + count_orbits(&orbits, "SAN", target) - 2
}

pub struct Orbit {
    orbitee: String,
    orbiter: String,
}

impl FromStr for Orbit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(')').collect();
        if parts.len() != 2 { return Err(()) }
        Ok(Orbit { orbitee: parts[0].to_string(), orbiter: parts[1].to_string() })
    }
}

fn to_orbits(input: Vec<Orbit>) -> HashMap<String, String> {
    input.into_iter().map(|it| (it.orbiter, it.orbitee)).collect()
}

fn count_orbits(orbits: &HashMap<String, String>, object: &str, target: &str) -> u32 {
    OrbitChain::new(orbits, object)
        .enumerate()
        .find(|(_, it)| it == &target)
        .map(|(i, _)| (i + 1) as u32)
        .expect(&format!("{} does not orbit {}", object, target))
}

fn  nearest_common_parent<'a>(orbits: &'a HashMap<String, String>, a: &'a str, b: &'a str) -> &'a str {
    let mut chain_a = OrbitChain::new(orbits, a);
    let chain_b: HashSet<_> = OrbitChain::new(orbits, b).collect();
    chain_a.find(|it| chain_b.contains(it)).expect("Objects must both orbit COM")
}

struct OrbitChain<'a> {
    orbits: &'a HashMap<String, String>,
    current: Option<&'a str>,
}

impl<'a> OrbitChain<'a> {
    fn new(orbits: &'a HashMap<String, String>, object: &'a str) -> OrbitChain<'a> {
        OrbitChain {
            orbits,
            current: Some(object),
        }
    }
}

impl<'a> Iterator for OrbitChain<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = match self.orbits.get(current) {
            Some(v) => Some(v),
            None => None
        };
        self.current
    }
}
