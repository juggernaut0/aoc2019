use std::str::FromStr;
use itertools::Itertools;
use std::collections::HashMap;
use std::f64::consts::PI;

pub fn run1(input: Vec<MapRow>) -> usize {
    let height = input.len();
    let width = input[0].0.len();
    itertools::iproduct!(0..width, 0..height)
        .filter(|&target| get(&input, target) == Tile::Asteroid)
        .map(|start| {
            let c = count_asteroids(&input, width, height, start);
            log::debug!("start: {:?} count: {}", start, c);
            c
        })
        .max()
        .unwrap()
}

pub fn run2(input: Vec<MapRow>) -> Point {
    let height = input.len();
    let width = input[0].0.len();

    // Find the laser (using part one's algorithm)
    let laser: Point = itertools::iproduct!(0..width, 0..height)
        .filter(|&target| get(&input, target) == Tile::Asteroid)
        .max_by_key(|&start| count_asteroids(&input, width, height, start))
        .unwrap();
    log::info!("laser: {:?}", laser);

    // get groups of asteroids in a line
    let mut groups: HashMap<(isize, isize), Vec<Point>> = HashMap::new();
    itertools::iproduct!(0..width, 0..height)
        .filter(|&target| get(&input, target) == Tile::Asteroid)
        .filter(|&target| laser != target)
        .for_each(|target| {
            let slope = get_slope(laser, target);
            if let Some(v) = groups.get_mut(&slope) {
                v.push(target);
            } else {
                groups.insert(slope, vec![target]);
            }
        });

    // Sort keys in order of angle from start
    let ordered: Vec<(isize, isize)> = groups.keys()
        .sorted_by(|&&a, &&b| get_angle(a).partial_cmp(&get_angle(b)).unwrap())
        .copied()
        .collect();

    // Round robin through the angles, counting up to 200
    let mut i = 0;
    for k in ordered.iter().cycle() {
        if groups.values().all(|it| it.is_empty()) {
            panic!("Ran out of items")
        }
        if let Some(v) = groups.get_mut(k) {
            if let Some(vaporized) = v.pop() {
                i += 1;
                if i == 200 {
                    return vaporized
                }
            }
        }
    }
    unreachable!()
}

pub struct MapRow(Vec<Tile>);

impl FromStr for MapRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MapRow(s.chars().map(|it| Tile::from_char(it)).collect()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Blank,
    Asteroid,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Blank,
            '#' => Tile::Asteroid,
            _ => panic!("Unknown tile: {}", c)
        }
    }
}

type Point = (usize, usize);

fn get(map: &Vec<MapRow>, (x, y): Point) -> Tile {
    (&map[y].0)[x]
}

fn count_asteroids(map: &Vec<MapRow>, width: usize, height: usize, start: Point) -> usize {
    itertools::iproduct!(0..width, 0..height)
        .filter(|&target| get(map, target) == Tile::Asteroid)
        .filter(|&target| start != target && can_see(map, width, height, start, target))
        .count()
}

fn can_see(map: &Vec<MapRow>, width: usize, height: usize, start: Point, target: Point) -> bool {
    assert_ne!(start, target);
    let dx = target.0 as isize - start.0 as isize;
    let dy = target.1 as isize - start.1 as isize;
    let sx = if dx == 0 { 0 } else if dy == 0 { dx / dx.abs() } else { dx / gcd(dx.abs(), dy.abs()) };
    let sy = if dy == 0 { 0 } else if dx == 0 { dy / dy.abs() } else { dy / gcd(dx.abs(), dy.abs()) };

    let mut x = add(start.0, sx);
    let mut y = add(start.1, sy);
    log::trace!("start = {:?}, target = {:?}, step = {:?}", start, target, (sx, sy));
    while (x, y) != target && x < width && y < height {
        if get(map, (x, y)) != Tile::Blank {
            return false
        }
        x = add(x, sx);
        y = add(y, sy);
    }
    return true
}

fn add(a: usize, b: isize) -> usize {
    ((a as isize) + b) as usize
}

fn gcd(a: isize, b: isize) -> isize {
    if a > b {
        gcd(a - b, b)
    } else if a < b {
        gcd(a, b - a)
    } else {
        a
    }
}

fn get_slope(start: Point, target: Point) -> (isize, isize) {
    let dx = target.0 as isize - start.0 as isize;
    let dy = target.1 as isize - start.1 as isize;
    let sx = if dx == 0 { 0 } else if dy == 0 { dx / dx.abs() } else { dx / gcd(dx.abs(), dy.abs()) };
    let sy = if dy == 0 { 0 } else if dx == 0 { dy / dy.abs() } else { dy / gcd(dx.abs(), dy.abs()) };
    (sx, sy)
}

fn get_angle((dx, dy): (isize, isize)) -> f64 {
    // laser starts pointing up and rotates clockwise
    if dx == 0 && dy < 0 {
        0.0
    } else if dx == 0 && dy > 0 {
        PI
    } else if dx > 0 {
        stoa(dx, dy)
    } else { // dx < 0
        PI + stoa(dx, dy)
    }
}

fn stoa(dx: isize, dy: isize) -> f64 {
    (PI / 2.0) + (dy as f64 / dx as f64).atan()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_input;

    #[test]
    fn gcd_test() {
        assert_eq!(4, gcd(12, 8));
        assert_eq!(4, gcd(8, 12));
    }

    #[test]
    fn simple() {

        let map = parse_input(".#..#
.....
#####
....#
...##");
        assert_eq!(true, can_see(&map, 5, 5, (0, 2), (1, 2)));
        assert_eq!(false, can_see(&map, 5, 5, (0, 2), (2, 2)));
        assert_eq!(false, can_see(&map, 5, 5, (4, 4), (4, 2)));
        assert_eq!(6, count_asteroids(&map, 5, 5, (0, 2)));
        assert_eq!(8, run1(map));
    }

    #[test]
    fn pt1ex1() {
        let map = parse_input("......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####");
        assert_eq!(33, run1(map));
    }
}
