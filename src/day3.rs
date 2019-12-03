use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;

pub fn run1(input: Vec<Wire>) -> u32 {
    let av = to_lines(&input[0].0);
    log::debug!("av: {:?}", av);
    let bv = to_lines(&input[1].0);
    av.iter()
        .cartesian_product(&bv)
        .filter_map(|(a, b)| find_intersection(a, b))
        .map(|it| distance((0, 0), it.point))
        .min()
        .unwrap_or(0)
}

pub fn run2(input: Vec<Wire>) -> u32 {
    let av = to_lines(&input[0].0);
    let bv = to_lines(&input[1].0);
    log::debug!("av: {:?}", av);
    log::debug!("bv: {:?}", bv);
    av.iter()
        .cartesian_product(&bv)
        .filter_map(|(a, b)| find_intersection(a, b))
        .map(|it| delay(it))
        .min()
        .unwrap_or(0)
}

pub struct Wire(Vec<Segment>);
pub struct Segment {
    dir: Direction,
    length: i32,
}
#[derive(Eq, PartialEq)]
pub enum Direction {
    Right, Up, Left, Down
}

impl FromStr for Wire {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Result<Vec<_>, _> = s.split(',').map(|it| it.parse()).collect();
        Ok(Wire(segments?))
    }
}

impl FromStr for Segment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s.chars().next() {
            Some('R') => Direction::Right,
            Some('U') => Direction::Up,
            Some('L') => Direction::Left,
            Some('D') => Direction::Down,
            _ => unreachable!("lol")
        };
        let length = s[1..].parse()?;
        Ok(Segment { dir, length })
    }
}

type Point = (i32, i32);
#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
    start_dist: i32,
    end_dist: i32,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end, start_dist: 0, end_dist: 0 }
    }

    fn from_segment(prev: &Line, segment: &Segment) -> Line {
        let start = prev.end;
        let end = match segment.dir {
            Direction::Right => (start.0 + segment.length, start.1),
            Direction::Up => (start.0, start.1 + segment.length),
            Direction::Left => (start.0 - segment.length, start.1),
            Direction::Down => (start.0, start.1 - segment.length),
        };
        let start_dist = prev.end_dist;
        let end_dist = start_dist + segment.length;
        Line { start, end, start_dist, end_dist }
    }

    fn vertical(&self) -> bool {
        self.start.0 == self.end.0
    }
}

fn to_lines(segments: &Vec<Segment>) -> Vec<Line> {
    segments.iter()
        .scan(Line::new((0, 0), (0, 0)), |state, it| {
            let line = Line::from_segment(state, it);
            *state = line.clone();
            Some(line)
        })
        .collect()
}

#[derive(Debug)]
struct Intersection {
    point: Point,
    line1: Line,
    line2: Line,
}

fn find_intersection(a: &Line, b: &Line) -> Option<Intersection> {
    if a.vertical() && b.vertical() {
        if a.start.0 == b.start.0 && (
            is_in(a.start.1, b.start.1, b.end.1)
                || is_in(a.end.1, b.start.1, b.end.1)
                || is_in(b.end.1, a.start.1, a.end.1)
                || is_in(b.end.1, a.start.1, a.end.1)
        ) {
            unimplemented!("colinear y: {:?} {:?}", a, b);
        } else {
            None
        }
    } else if !a.vertical() && !b.vertical() {
        if a.start.1 == b.start.1 && (
            is_in(a.start.0, b.start.0, b.end.0)
                || is_in(a.end.0, b.start.0, b.end.0)
                || is_in(b.start.0, a.start.0, a.end.0)
                || is_in(b.end.0, a.start.0, a.end.0)
        ) {
            unimplemented!("colinear x: {:?} {:?}", a, b);
        } else {
            None
        }
    } else if a.vertical() && is_in(a.start.0, b.start.0, b.end.0) && is_in(b.start.1, a.start.1, a.end.1) {
        Some(Intersection { point: (a.start.0, b.start.1), line1: a.clone(), line2: b.clone() })
    } else if b.vertical() && is_in(a.start.1, b.start.1, b.end.1) && is_in(b.start.0, a.start.0, a.end.0) {
        Some(Intersection { point: (b.start.0, a.start.1), line1: a.clone(), line2: b.clone() })
    } else {
        None
    }
}

fn is_in(x: i32, a: i32, b: i32) -> bool {
    (x - a) * (x - b) < 0
}

fn distance(a: Point, b: Point) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

fn delay(intersection: Intersection) -> u32 {
    log::debug!("intersection: {:?}", intersection);
    let l1d = distance(intersection.point, intersection.line1.start) + (intersection.line1.start_dist as u32);
    let l2d = distance(intersection.point, intersection.line2.start) + (intersection.line2.start_dist as u32);
    l1d + l2d
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_input;

    #[test]
    fn intersect() {
        assert_eq!(Some((2, 3)), find_intersection(&Line::new((2, 5), (2, 1)), &Line::new((0, 3), (9, 3))).map(|it| it.point));
        assert_eq!(None, find_intersection(&Line::new((2, 5), (2, 1)), &Line::new((0, 3), (1, 3))).map(|it| it.point));
        assert_eq!(None, find_intersection(&Line::new((2, -5), (2, 1)), &Line::new((0, 3), (9, 3))).map(|it| it.point));
    }

    #[test]
    fn pt1ex1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(159, run1(parse_input(input)));
    }

    #[test]
    fn pt2ex1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(610, run2(parse_input(input)));
    }
}
