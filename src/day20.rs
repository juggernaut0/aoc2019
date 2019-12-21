use std::collections::{HashMap, VecDeque};

pub fn run1(input: Vec<String>) -> u32 {
    let map = Map::from_input(input);

    pathfind(&map.chars, &map.outside_portals, &map.inside_portals)
}

pub fn run2(input: Vec<String>) -> u32 {
    let map = Map::from_input(input);

    pathfind2(&map.chars, &map.outside_portals, &map.inside_portals)
}

struct Map {
    chars: HashMap<(usize, usize), char>,
    outside_portals: HashMap<(usize, usize), String>,
    inside_portals: HashMap<(usize, usize), String>,
}

impl Map {
    fn from_input(input: Vec<String>) -> Map {
        let width = input[0].len() - 4;
        let height = input.len() - 4;
        let thicc = input.iter()
            .skip(2)
            .enumerate()
            .find(|(_, row)| {
                let l = row.len();
                row[2..l-2].contains(' ')
            })
            .map(|(i, _)| i)
            .expect("Expected a donut");

        let mut chars = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                chars.insert((x, y), c);
            }
        }

        let mut outside_portals = HashMap::new();
        let mut inside_portals = HashMap::new();
        // top outside
        for x in 2..2+width {
            let y = 2;
            if chars[&(x, y)] == '.' {
                let mut s = String::new();
                s.push(chars[&(x, y-2)]);
                s.push(chars[&(x, y-1)]);
                outside_portals.insert((x, y), s);
            }
        }
        // bottom outside
        for x in 2..2+width {
            let y = height+1;
            if chars[&(x, y)] == '.' {
                let mut s = String::new();
                s.push(chars[&(x, y+1)]);
                s.push(chars[&(x, y+2)]);
                outside_portals.insert((x, y), s);
            }
        }
        // top inside
        for x in 2+thicc..2+width-thicc {
            let y = thicc+1;
            if chars[&(x, y)] == '.' {
                let mut s = String::new();
                s.push(chars[&(x, y+1)]);
                s.push(chars[&(x, y+2)]);
                inside_portals.insert((x, y), s);
            }
        }
        // bottom inside
        for x in 2+thicc..2+width-thicc {
            let y = height-thicc+2;
            if chars[&(x, y)] == '.' {
                let mut s = String::new();
                s.push(chars[&(x, y-2)]);
                s.push(chars[&(x, y-1)]);
                inside_portals.insert((x, y), s);
            }
        }
        // left outside
        for y in 2..2+height {
            let x = 2;
            if chars[&(x, y)] == '.' {
                let mut s = String::new();
                s.push(chars[&(x-2, y)]);
                s.push(chars[&(x-1, y)]);
                outside_portals.insert((x, y), s);
            }
        }
        // right outside
        for y in 2..2+height {
            let x = width+1;
            if chars[&(x, y)] == '.' {
                let mut s = String::new();
                s.push(chars[&(x+1, y)]);
                s.push(chars[&(x+2, y)]);
                outside_portals.insert((x, y), s);
            }
        }
        // left inside
        for y in 2+thicc..2+height-thicc {
            let x = thicc+1;
            if chars[&(x, y)] == '.' {
                let mut s = String::new();
                s.push(chars[&(x+1, y)]);
                s.push(chars[&(x+2, y)]);
                inside_portals.insert((x, y), s);
            }
        }
        // right inside
        for y in 2+thicc..2+height-thicc {
            let x = width-thicc+2;
            if chars[&(x, y)] == '.' {
                let mut s = String::new();
                s.push(chars[&(x-2, y)]);
                s.push(chars[&(x-1, y)]);
                inside_portals.insert((x, y), s);
            }
        }

        assert_eq!(outside_portals.len(), inside_portals.len() + 2);

        Map { chars, outside_portals, inside_portals }
    }
}

fn pathfind(
    chars: &HashMap<(usize, usize), char>,
    outside_portals: &HashMap<(usize, usize), String>,
    inside_portals: &HashMap<(usize, usize), String>,
) -> u32 {
    let start = outside_portals.iter().find(|it| it.1 == "AA").map(|(&pos, _)| pos).unwrap();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut costs = HashMap::new();
    costs.insert(start, 0);
    /*let mut outside_d = HashMap::new(); // dist from start to outside portal
    let mut inside_d = HashMap::new();*/
    while let Some(pos) = queue.pop_front() {
        let cost = costs[&pos];
        if let Some(p) = inside_portals.get(&pos) {
            let (adj, _) = outside_portals.iter()
                .find(|(_, op)| *op == p)
                .unwrap();
            let new_cost = cost + 1;
            let old_cost = costs.get(adj).copied().unwrap_or(999999999);
            if new_cost < old_cost {
                queue.push_back(*adj);
                costs.insert(*adj, new_cost);
            }
        } else if let Some(p) = outside_portals.get(&pos) {
            if p != "ZZ" && p != "AA" {
                let (adj, _) = inside_portals.iter()
                    .find(|(_, op)| *op == p)
                    .expect("Expected to find a matching inside portal");
                let new_cost = cost + 1;
                let old_cost = costs.get(adj).copied().unwrap_or(999999999);
                if new_cost < old_cost {
                    queue.push_back(*adj);
                    costs.insert(*adj, new_cost);
                }
            }
        }
        for adj in adj(pos).iter() {
            let new_cost = cost + 1;
            let old_cost = costs.get(adj).copied().unwrap_or(999999999);
            let can_move = chars[adj] == '.';
            if can_move && new_cost < old_cost {
                queue.push_back(*adj);
                costs.insert(*adj, new_cost);
            }
        }
    }

    let portal_costs: HashMap<_, _> = outside_portals.iter()
        .map(|(pos, name)| (name, costs.get(pos)))
        .collect();
    log::info!("{:#?}", portal_costs);

    outside_portals.iter()
        .find(|it| it.1 == "ZZ")
        .map(|(zz_pos, _)| costs[zz_pos])
        .expect("Epected to find ZZ")
}

fn pathfind2(
        chars: &HashMap<(usize, usize), char>,
        outside_portals: &HashMap<(usize, usize), String>,
        inside_portals: &HashMap<(usize, usize), String>,
) -> u32 {
    let start = outside_portals.iter().find(|it| it.1 == "AA").map(|(&pos, _)| pos).unwrap();
    let goal = outside_portals.iter()
        .find(|it| it.1 == "ZZ")
        .map(|(&zz_pos, _)| zz_pos)
        .expect("Epected to find ZZ");
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut costs = HashMap::new();
    costs.insert((start, 0), 0);
    while let Some(current) = queue.pop_front() {
        let cost = costs[&current];
        if current == (goal, 0) {
            return cost;
        }
        let (pos, layer) = current;
        if let Some(p) = inside_portals.get(&pos) {
            let (&dest, _) = outside_portals.iter()
                .find(|(_, op)| *op == p)
                .unwrap();
            let adj = (dest, layer + 1);
            let new_cost = cost + 1;
            let old_cost = costs.get(&adj).copied().unwrap_or(999999999);
            if new_cost < old_cost {
                queue.push_back(adj);
                costs.insert(adj, new_cost);
            }
        } else if let Some(p) = outside_portals.get(&pos) {
            if layer > 0 && p != "ZZ" && p != "AA" {
                let (&dest, _) = inside_portals.iter()
                    .find(|(_, op)| *op == p)
                    .expect("Expected to find a matching inside portal");
                let adj = (dest, layer - 1);
                let new_cost = cost + 1;
                let old_cost = costs.get(&adj).copied().unwrap_or(999999999);
                if new_cost < old_cost {
                    queue.push_back(adj);
                    costs.insert(adj, new_cost);
                }
            }
        }
        for &dest in adj(pos).iter() {
            let adj = (dest, layer);
            let new_cost = cost + 1;
            let old_cost = costs.get(&adj).copied().unwrap_or(999999999);
            let can_move = chars[&dest] == '.';
            if can_move && new_cost < old_cost {
                queue.push_back(adj);
                costs.insert(adj, new_cost);
            }
        }
    }

    unreachable!("Ran out of places to go")
}

fn adj((x, y): (usize, usize)) -> [(usize, usize);4] {
    [
        (x + 1, y),
        (x, y + 1),
        (x - 1, y),
        (x, y - 1),
    ]
}
