use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use std::cmp::min;

pub fn run1(input: Vec<String>) -> u32 {
    let map = parse_map(input);
    let keys = map.keys();
    let mut cache = HashMap::new();
    let mut nodes: HashMap<char, HashMap<String, u32>> = HashMap::new();
    keys.iter()
        .copied()
        .filter_map(|k| cached_pathfind(&map, &mut cache, map.starts[0], find_key(&map, k), &HashSet::new()).map(|cost| (k, cost)))
        .for_each(|(k, cost)| {
            let mut v = HashMap::new();
            v.insert(String::new(), cost);
            nodes.insert(k, v);
        });
    for _ in 0..keys.len()-1 {
        log::info!("{:#?}", nodes);
        let mut new_nodes = HashMap::new();
        for &key in &keys {
            let mut new_paths = HashMap::new();
            for (&end_key, paths) in &nodes {
                for (via, &cost) in paths {
                    let mut subset = str_to_set(&via);
                    subset.insert(end_key);
                    if !subset.contains(&key) {
                        if let Some(segment_cost) = cached_pathfind(&map, &mut cache, find_key(&map, end_key), find_key(&map, key), &subset) {
                            let subset_str = set_to_string(&subset);
                            if let Some(current) = new_paths.get(&subset_str).copied() {
                                new_paths.insert(subset_str, min(current, cost + segment_cost));
                            } else {
                                new_paths.insert(subset_str, cost + segment_cost);
                            }
                        }
                    }
                }
            }
            if !new_paths.is_empty() {
                new_nodes.insert(key, new_paths);
            }
        }
        nodes = new_nodes;
    }

    log::info!("{:#?}", nodes);

    nodes.values().flat_map(|it| it.values()).min().copied().unwrap()
}

pub fn run2(input: Vec<String>) -> u32 {
    let map = parse_map(input);
    let keys = map.keys();
    let mut cache = HashMap::new();
    let pkeys = {
        let all_keys: HashSet<char> = keys.iter().copied().collect();
        let mut parts = [Vec::new(),Vec::new(),Vec::new(),Vec::new()];
        for &k in &keys {
            for (i, start) in map.starts.iter().copied().enumerate() {
                if cached_pathfind(&map, &mut cache, start, find_key(&map, k), &all_keys).is_some() {
                    parts[i].push(k);
                }
            }
        }
        parts[0].push('0');
        parts[1].push('1');
        parts[2].push('2');
        parts[3].push('3');
        parts
    };

    log::debug!("{:?}", pkeys);

    let mut nodes: HashMap<[char;4], HashMap<String, u32>> = HashMap::new();
    {
        let mut v = HashMap::new();
        v.insert(String::new(), 0);
        nodes.insert(['0', '1', '2', '3'], v);
    }

    for _ in 0..keys.len() {
        log::info!("{:#?}", nodes);
        let mut new_nodes: HashMap<[char;4], HashMap<String, u32>> = HashMap::new();
        for &key in &keys {
            let p = pkeys.iter().enumerate().find(|it| it.1.contains(&key)).expect("key not found in partitions").0;
            for (&end_keys, paths) in &nodes {
                let mut new_paths = HashMap::new();
                for (via, &cost) in paths {
                    let mut held_keys = str_to_set(&via);
                    for &end_key in &end_keys {
                        held_keys.insert(end_key);
                    }

                    if !held_keys.contains(&key) {
                        let new_cost = cached_pathfind(&map, &mut cache, find_key(&map, end_keys[p]), find_key(&map, key), &held_keys);
                        if let Some(segment_cost) = new_cost {
                            let subset_str = set_to_string(&held_keys);
                            if let Some(current) = new_paths.get(&subset_str).copied() {
                                new_paths.insert(subset_str, min(current, cost + segment_cost));
                            } else {
                                new_paths.insert(subset_str, cost + segment_cost);
                            }
                        }
                    }
                }
                if !new_paths.is_empty() {
                    let mut ks = end_keys;
                    ks[p] = key;
                    if let Some(paths) = new_nodes.get_mut(&ks) {
                        for path in new_paths {
                            if let Some(current) = paths.get_mut(&path.0) {
                                *current = min(*current, path.1);
                            } else {
                                paths.insert(path.0, path.1);
                            }
                        }
                    } else {
                        new_nodes.insert(ks, new_paths);
                    }
                }
            }

        }
        nodes = new_nodes;
    }

    log::info!("{:#?}", nodes);

    nodes.values().flat_map(|it| it.values()).min().copied().unwrap()
}

type Cache = HashMap<CacheKey, Option<u32>>;

#[derive(Eq, PartialEq, Hash)]
struct CacheKey {
    start: (usize, usize),
    dest: (usize, usize),
    keys: String,
}

impl CacheKey {
    fn new(start: (usize, usize), dest: (usize, usize), keys: &HashSet<char>) -> CacheKey {
        CacheKey {
            start,
            dest,
            keys: set_to_string(keys),
        }
    }
}

struct Map {
    map: HashMap<(usize, usize), Tile>,
    starts: Vec<(usize, usize)>,
}

impl Map {
    fn keys(&self) -> Vec<char> {
        self.map.values()
            .filter_map(|&it| {
                if let Tile::Key(c) = it {
                    Some(c)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Key(char),
    Door(char),
}

fn parse_map(rows: Vec<String>) -> Map {
    let mut map = HashMap::new();
    let mut starts = Vec::new();
    for y in 0..rows.len() {
        let row = &rows[y];
        for (x, c) in row.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Empty,
                '@' => {
                    starts.push((x, y));
                    Tile::Empty
                },
                '#' => Tile::Wall,
                k if ('a'..='z').contains(&k) => Tile::Key(k),
                d if ('A'..='Z').contains(&d) => Tile::Door(d),
                _ => panic!("Unrecognized map character: {}", c)
            };
            map.insert((x, y), tile);
        }
    }
    Map { map, starts }
}

fn find_key(map: &Map, key: char) -> (usize, usize) {
    if let Some(i) = key.to_digit(10) {
        map.starts[i as usize]
    } else {
        map.map.iter()
            .find(|(_, &tile)| {
                match tile {
                    Tile::Key(k) => k == key,
                    _ => false
                }
            })
            .map(|(&key_pos, _)| key_pos)
            .expect("key not found")
    }
}

fn cached_pathfind(map: &Map, cache: &mut Cache, start: (usize, usize), dest: (usize, usize), held_keys: &HashSet<char>) -> Option<u32> {
    let cache_key = CacheKey::new(start, dest, held_keys);
    if !cache.contains_key(&cache_key) {
        for (dest, maybe_cost) in pathfind(map, start, held_keys) {
            cache.insert(CacheKey::new(start, dest, held_keys), maybe_cost);
        }
    }
    cache.get(&cache_key).copied().unwrap()
}

fn pathfind(map: &Map, start: (usize, usize), held_keys: &HashSet<char>) -> HashMap<(usize, usize), Option<u32>> {
    log::debug!("pathfind from {:?} with keys {:?}", start, held_keys);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut costs = HashMap::new();
    costs.insert(start, 0);
    while let Some(pos) = queue.pop_front() {
        let cost = costs[&pos];
        for &adj in adj(pos).iter() {
            let new_cost = cost + 1;
            let old_cost = costs.get(&adj).copied().unwrap_or(999999999);
            let can_move = match map.map[&adj] {
                Tile::Empty | Tile::Key(_) => true,
                Tile::Door(d) => held_keys.contains(&d.to_ascii_lowercase()),
                _ => false
            };
            if can_move && new_cost < old_cost {
                queue.push_back(adj);
                costs.insert(adj, new_cost);
            }
        }
    }
    map.map.iter()
        .filter(|(_, &tile)| {
            match tile {
                Tile::Key(_) => true,
                _ => false
            }
        })
        .map(|(key_pos, _)| key_pos)
        .chain(map.starts.iter())
        .map(|&key_pos| (key_pos, costs.get(&key_pos).copied()))
        .collect()
}

fn adj((x, y): (usize, usize)) -> [(usize, usize);4] {
    [
        (x + 1, y),
        (x, y + 1),
        (x - 1, y),
        (x, y - 1),
    ]
}

fn set_to_string(set: &HashSet<char>) -> String {
    set.iter().sorted().collect()
}

fn str_to_set(str: &str) -> HashSet<char> {
    str.chars().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_input;

    #[test]
    fn pathfind_test() {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        let map = parse_map(parse_input(input));
        assert_eq!(Some(Some(2)), pathfind(&map, (6, 3), &HashSet::new()).get(&(8, 3)).copied());
    }

    #[test]
    fn pt1ex1() {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        assert_eq!(132, run1(parse_input(input)));
    }

    #[test]
    fn pt2ex1() {
        let input = "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############";
        assert_eq!(32, run2(parse_input(input)));
    }

    #[test]
    fn pt2ex2() {
        let input = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";

        assert_eq!(72, run2(parse_input(input)));
    }
}
