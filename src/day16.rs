use itertools::Itertools;

pub fn run1(input: Vec<String>) -> String {
    let mut seq = parse(&input[0]);
    for _ in 0..100 {
        seq = compute_phase(seq);
    }
    seq[0..8].iter().join("")
}

pub fn run2(input: Vec<String>) -> String {
    let offset: usize = (&input[0]).chars().take(7).collect::<String>().parse().unwrap();
    let orig = parse(&input[0]);
    let mut seq: Vec<_> = orig.iter()
        .copied()
        .rev()
        .cycle()
        .take(orig.len() * 10000 - offset)
        .collect();

    for i in 0..100 {
        log::info!("iteration: {}", i + 1);
        seq = seq.iter()
            .scan(0, |a, &b| {
                *a += b as i32;
                Some((a.abs() % 10) as i8)
            })
            .collect();
    }
    seq.iter().rev().take(8).join("")
}

fn parse(s: &str) -> Vec<i8> {
    s.chars().filter_map(|it| it.to_digit(10).map(|d| d as i8)).collect()
}

fn fft_pattern(length: usize, i: usize) -> i8 {
    let phase = ((i + 1) / length) % 4;
    match phase {
        0 => 0,
        1 => 1,
        2 => 0,
        3 => -1,
        _ => unreachable!()
    }
}

fn compute_phase(input: Vec<i8>) -> Vec<i8> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        let sum: i32 = input[i..].iter()
            .copied()
            .enumerate()
            .map(|(j, x)| (x * fft_pattern(i + 1, j + i)) as i32)
            .sum();
        result.push((sum.abs() % 10) as i8);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_input;

    #[test]
    fn pattern() {
        assert_eq!(vec![1, 0, -1, 0, 1, 0, -1, 0], (0..8).map(|i| fft_pattern(1, i)).collect::<Vec<_>>());
        assert_eq!(vec![0, 1, 1, 0, 0, -1, -1, 0], (0..8).map(|i| fft_pattern(2, i)).collect::<Vec<_>>());
        assert_eq!(vec![0, 0, 1, 1, 1, 0, 0, 0], (0..8).map(|i| fft_pattern(3, i)).collect::<Vec<_>>());
    }

    #[test]
    fn phase() {
        simple_logger::init().unwrap();
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let p1 = compute_phase(input);
        assert_eq!(vec![4,8,2,2,6,1,5,8], p1);
    }

    #[test]
    fn pt1ex1() {
        let input = parse_input("80871224585914546619083218645595");
        assert_eq!("24176176", run1(input));
    }
}
