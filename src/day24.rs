use std::collections::{HashMap, HashSet};

pub fn run1(input: Vec<String>) -> u64 {
    let mut map = parse_map(&input);
    let mut prevs = HashSet::new();
    loop {
        let bdr = calc_biodiversity(&map);
        log::debug!("Biodiverstiy: {}", bdr);
        if !prevs.insert(bdr) {
            return bdr;
        }
        step1(&mut map);
    }
}

pub fn run2(input: Vec<String>) -> usize {
    let mut map = parse_map(&input);
    for _ in 0..200 {
        step2(&mut map);
    }
    map.keys().count()
}

type Map = HashMap<(i32, i32, i32), Tile>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Bug,
}

fn parse_map(input: &[String]) -> Map {
    let mut map = HashMap::new();
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Empty,
                '#' => Tile::Bug,
                _ => panic!("Unknown tile: {}", c)
            };
            map.insert((x as i32, y as i32, 0), tile);
        }
    }
    map
}

fn step1(map: &mut Map) {
    let mut new_map = HashMap::new();
    for y in 0..5 {
        for x in 0..5 {
            let coords = (x, y, 0);
            let tile = map.get(&coords).copied().unwrap_or(Tile::Empty);
            let adj_bugs = count_adj_bugs(&map, adj1, coords);
            if tile == Tile::Bug && adj_bugs == 1 {
                new_map.insert(coords, Tile::Bug);
            } else if tile == Tile::Empty && (adj_bugs == 1 || adj_bugs == 2) {
                new_map.insert(coords, Tile::Bug);
            }
        }
    }
    *map = new_map;
}

fn step2(map: &mut Map) {
    let mut new_map = HashMap::new();
    let min_level = map.keys().map(|&(_, _, l)| l).min().unwrap() - 1;
    let max_level = map.keys().map(|&(_, _, l)| l).max().unwrap() + 1;
    for level in min_level..=max_level {
        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }
                let coords = (x, y, level);
                let tile = map.get(&coords).copied().unwrap_or(Tile::Empty);
                let adj_bugs = count_adj_bugs(&map, adj2, coords);
                if tile == Tile::Bug && adj_bugs == 1 {
                    new_map.insert(coords, Tile::Bug);
                } else if tile == Tile::Empty && (adj_bugs == 1 || adj_bugs == 2) {
                    new_map.insert(coords, Tile::Bug);
                }

            }
        }
    }
    *map = new_map;
}


fn count_adj_bugs(map: &Map, mut adj_fn: impl AdjFn, (x, y, level): (i32, i32, i32)) -> usize {
    adj_fn(x, y, level).iter()
        .filter(|&it| {
            map.get(it).copied().unwrap_or(Tile::Empty) == Tile::Bug
        })
        .count()
}

trait AdjFn : FnMut(i32, i32, i32) -> Vec<(i32, i32, i32)> {}
impl<T: FnMut(i32, i32, i32) -> Vec<(i32, i32, i32)>> AdjFn for T {}

fn adj1(x: i32, y: i32, level: i32) -> Vec<(i32, i32, i32)> {
    vec![
        (x + 1, y, level),
        (x, y + 1, level),
        (x - 1, y, level),
        (x, y - 1, level),
    ]
}

fn adj2(x: i32, y: i32, level: i32) -> Vec<(i32, i32, i32)> {
    assert_ne!((2, 2), (x, y));
    let mut res = Vec::new();
    // left
    if x == 0 {
        res.push((1, 2, level - 1));
    } else if x == 3 && y == 2 {
        for iy in 0..5 {
            res.push((4, iy, level + 1));
        }
    } else {
        res.push((x - 1, y, level));
    }
    // right
    if x == 4 {
        res.push((3, 2, level - 1))
    } else if x == 1 && y == 2 {
        for iy in 0..5 {
            res.push((0, iy, level + 1));
        }
    } else {
        res.push((x + 1, y, level))
    }
    // up
    if y == 0 {
        res.push((2, 1, level - 1));
    } else if x == 2 && y == 3 {
        for ix in 0..5 {
            res.push((ix, 4, level + 1));
        }
    } else {
        res.push((x, y - 1, level));
    }
    // down
    if y == 4 {
        res.push((2, 3, level - 1));
    } else if x == 2 && y == 1 {
        for ix in 0..5 {
            res.push((ix, 0, level + 1));
        }
    } else {
        res.push((x, y + 1, level));
    }
    res
}

fn calc_biodiversity(map: &Map) -> u64 {
    let mut res = 0;
    for y in 0..5 {
        for x in 0..5 {
            if let Some(Tile::Bug) = map.get(&(x, y, 0)) {
                res += 1 << (y*5 + x) as u64
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn bdr() {
        let input = ".....
.....
.....
#....
.#...";
        let map = parse_map(&input.lines().map(|s| s.to_string()).collect_vec());
        let bdr = calc_biodiversity(&map);
        assert_eq!(2129920, bdr)
    }
}
