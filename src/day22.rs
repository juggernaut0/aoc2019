use std::collections::VecDeque;
use std::str::FromStr;
use std::num::ParseIntError;
use crate::day22::Technique::*;

use modinverse::modinverse;

pub fn run1(input: Vec<Technique>) -> usize {
    let deck = shuffle(10007, input);
    deck.iter().enumerate().find(|&(_, &v)| v == 2019).unwrap().0
}

pub fn run2(input: Vec<Technique>) -> u64 {
    lazy_shuffle(119315717514047, 2020, &input, 101741582076661)
}

fn shuffle(deck_size: u64, input: Vec<Technique>) -> Vec<u64> {
    let mut deck = VecDeque::new();
    for x in 0..deck_size {
        deck.push_back(x);
    }
    log::debug!("{:?}", deck);
    for t in input {
        log::info!("Doing {:?}...", t);
        t.execute(&mut deck);
        log::debug!("{:?}", deck);
    }

    deck.into()
}

fn lazy_shuffle(deck_size: u64, i: u64, input: &Vec<Technique>, times: u64) -> u64 {
    let (a, b) = repeat(fold_techniques(input, deck_size), deck_size, times);
    log::info!("a = {}, b = {}", a, b);
    (a*i + b) % deck_size
}

#[derive(Debug, Clone)]
pub enum Technique {
    NewStack,
    Cut(i64),
    Increment(usize),
}

impl FromStr for Technique {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "deal into new stack" {
            Ok(NewStack)
        } else if s.starts_with("cut") {
            let n = s.split(' ')
                .skip(1)
                .next()
                .ok_or_else(|| "need a number for cut".to_string())?
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?;
            Ok(Cut(n))
        } else if s.starts_with("deal with increment") {
            let n = s.split(' ')
                .skip(3)
                .next()
                .ok_or_else(|| "need a number for cut".to_string())?
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?;
            Ok(Increment(n))
        } else {
            Err(format!("Unrecognized technique: {}", s))
        }
    }
}

impl Technique {
    fn execute(&self, deck: &mut VecDeque<u64>) {
        let size = deck.len();
        let mut vec = Vec::with_capacity(size);
        let size = size as u64;
        for i in 0..size {
            let (a, b) = self.coefs(size);
            let di = (a*i + b) % size;
            vec.push(deck[di as usize]);
        }
        *deck = vec.into()
    }

    fn coefs(&self, size: u64) -> (u64, u64) {
        match self {
            NewStack => (size - 1, size - 1),
            &Cut(n) => (1, (n + size as i64) as u64 % size),
            &Increment(n) => (modinverse(n as i64, size as i64).unwrap() as u64, 0),
        }
    }
}

fn fold_techniques(techs: &Vec<Technique>, size: u64) -> (u64, u64) {
    techs.iter().fold((1, 0), |cs, t| compose(cs, t.coefs(size), size))
}

fn repeat(base: (u64, u64), size: u64, times: u64) -> (u64, u64) {
    if times == 1 {
        return base
    }
    let mut t = 1;
    let mut cs = base;
    while t * 2 <= times {
        cs = compose(cs, cs, size);
        t *= 2;
    }
    let c2s = repeat(base, size, times - t);
    compose(cs, c2s, size)
}

fn compose(first: (u64, u64), second: (u64, u64), size: u64) -> (u64, u64) {
    // c(ai + b) + d = aci + cb + d
    let (a, b) = second;
    let a = a as u128;
    let b = b as u128;
    let (c, d) = first;
    let c = c as u128;
    let d = d as u128;
    let newa = (a * c) % (size as u128);
    let newb = (c*b + d) % (size as u128);
    let result = (newa as u64, newb as u64);
    log::debug!("compose({:?}, {:?}) = {:?}", first, second, result);
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_input;

    #[test]
    fn new_stack() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into();
        Technique::NewStack.execute(&mut deck);
        assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], Vec::from(deck));
    }

    #[test]
    fn cut() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into();
        Technique::Cut(3).execute(&mut deck);
        assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], Vec::from(deck));
    }

    #[test]
    fn cut_neg() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into();
        Technique::Cut(-4).execute(&mut deck);
        assert_eq!(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], Vec::from(deck));
    }

    #[test]
    fn increment() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into();
        Technique::Increment(3).execute(&mut deck);
        assert_eq!(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3], Vec::from(deck));
    }

    #[test]
    fn pt1ex1() {
        let input = "deal with increment 7
deal into new stack
deal into new stack";
        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], shuffle(10, parse_input(input)));
    }

    #[test]
    fn pt1ex2() {
        let input = "cut 6
deal with increment 7
deal into new stack";
        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], shuffle(10, parse_input(input)));
    }

    #[test]
    fn pt1ex2coefs() {
        let input = "cut 6
deal with increment 7
deal into new stack";
        assert_eq!((7, 3), fold_techniques(&parse_input(input), 10));
        // 0 1 2 3 4 5 6 7 8 9 = 1i + 0
        // 6 7 8 9 0 1 2 3 4 5 = 1(1i + 6) + 0 = 1*1i + 1*6 + 0 = 1i + 6
        // 6 9 2 5 8 1 4 7 0 3 = 1*(modinv(7, 10)*i) + 6 = 1*3*i + 6 = 3i + 6
        // 3 0 7 4 1 8 5 2 9 6 = 3(9i + 9) + 6 = (27i + 27) + 6 == 27i + 33 = 7i + 3
    }

    #[test]
    fn pt1ex3() {
        let input = "deal with increment 7
deal with increment 9
cut -2";
        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], shuffle(10, parse_input(input)));
        // 1i + 0
        // 1*(modinv(7, 10)*i) + 0 = 3i + 0
        // 3*(modinv(9, 10)*i) + 0 = 3*9*i + 0 = 27i + 0 = 7i + 0
        // 7*(i - 2) + 0 = 7i - 14 + 0 = 7i - 14 = 7i + 6
    }

    #[test]
    fn pt1ex4() {
        let input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], shuffle(10, parse_input(input)));
    }
}
