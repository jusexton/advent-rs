use std::collections::{HashMap, HashSet};

fn get_antenna_coords(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    let shape = input.find("\n").unwrap();
    let map: String = input.chars().filter(|c| *c != '\n').collect();
    map.char_indices()
        .filter(|c| c.1 != '.')
        .fold(HashMap::new(), |mut acc, curr| {
            let x = (curr.0 % shape) as i32;
            let y = (curr.0 / shape) as i32;
            acc.entry(curr.1).or_default().push((x, y));
            acc
        })
}

fn find_anti_node(coordinate: (i32, i32), other: (i32, i32)) -> (i32, i32) {
    let (dx, dy) = (
        coordinate.0.abs_diff(other.0) as i32,
        coordinate.1.abs_diff(other.1) as i32,
    );

    let x = if coordinate.0 >= other.0 {
        coordinate.0 + dx
    } else {
        coordinate.0 - dx
    };
    let y = if coordinate.1 >= other.1 {
        coordinate.1 + dy
    } else {
        coordinate.1 - dy
    };

    (x, y)
}

fn on_map(coord: (i32, i32), shape: i32) -> bool {
    coord.0 >= 0 && coord.0 <= shape && coord.1 >= 0 && coord.1 <= shape
}

pub fn count_anti_nodes(input: &str) -> u32 {
    let shape = (input.find("\n").unwrap() - 1) as i32;
    let antenna_coords = get_antenna_coords(input);
    let mut node_coords = HashSet::new();
    for coords in antenna_coords.values() {
        for i in 0..coords.len() {
            for j in 0..coords.len() {
                if i == j {
                    continue;
                }
                let node = find_anti_node(coords[i], coords[j]);
                if on_map(node, shape) {
                    node_coords.insert(node);
                }
            }
        }
    }

    node_coords.len() as u32
}

pub fn count_resonant_anti_nodes(input: &str) -> u32 {
    let shape = (input.find("\n").unwrap() - 1) as i32;
    let antenna_coords = get_antenna_coords(input);
    let mut node_coords = HashSet::new();
    for coords in antenna_coords.values() {
        for i in 0..coords.len() {
            for j in 0..coords.len() {
                if i == j {
                    continue;
                }

                let mut coord = coords[i];
                let mut other = coords[j];
                loop {
                    let node = find_anti_node(coord, other);
                    if on_map(node, shape) {
                        node_coords.insert(node);
                        other = coord;
                        coord = node;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    for locations in antenna_coords.into_values() {
        if locations.len() > 1 {
            node_coords.extend(locations)
        }
    }

    node_coords.len() as u32
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2024 - Day 8) --- {}, {}",
        count_anti_nodes(input),
        count_resonant_anti_nodes(input)
    );
}
