use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Region {
    area: u32,
    perimeter: u32,
}

impl Region {
    fn new() -> Self {
        Self {
            area: 0,
            perimeter: 0,
        }
    }

    fn price(&self) -> u32 {
        self.area * self.perimeter
    }
}

fn adjacent_nodes(idx: usize, shape: usize, map: &[u8]) -> Vec<usize> {
    let (row, column) = (idx / shape, idx % shape);
    let mut nodes = vec![];

    // Left
    if column != 0 && map[idx] == map[idx - 1] {
        nodes.push(idx - 1);
    }

    // Right
    if column < shape - 1 && map[idx] == map[idx + 1] {
        nodes.push(idx + 1);
    }

    // Up
    if row > 0 && map[idx] == map[idx - shape] {
        nodes.push(idx - shape);
    }

    // Down
    if row < shape - 1 && map[idx] == map[idx + shape] {
        nodes.push(idx + shape);
    }

    nodes
}

pub fn garden_plot_price(input: &str) -> u32 {
    let shape = input.find('\n').unwrap();
    let map: Vec<u8> = input.bytes().filter(|&c| c != b'\n').collect();
    let mut visited = HashSet::new();
    let mut plots: HashMap<char, Vec<_>> = HashMap::new();
    for idx in 0..map.len() {
        if visited.contains(&idx) {
            continue;
        }

        let plant = map[idx] as char;
        let regions = plots.entry(plant).or_default();
        regions.push(Region::new());

        let mut stack = vec![idx];
        while let Some(node) = stack.pop() {
            visited.insert(node);
            let nodes = adjacent_nodes(node, shape, &map);

            let region = plots.get_mut(&plant).unwrap().last_mut().unwrap();
            region.area += 1;
            region.perimeter += 4 - nodes.len() as u32;

            stack.extend(nodes.iter().filter(|&&n| !visited.contains(&n)));
            visited.extend(nodes);
        }
    }

    plots.values().flatten().map(|region| region.price()).sum()
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2024 - Day 12) --- {}, INCOMPLETE",
        garden_plot_price(input)
    );
}
