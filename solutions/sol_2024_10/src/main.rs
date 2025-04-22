use std::collections::HashSet;

pub fn score_trail_heads(input: &str) -> u32 {
    let shape = input.find('\n').unwrap();
    let map: String = input.chars().filter(|&c| c != '\n').collect();
    map.char_indices()
        .filter(|(_, c)| *c == '0')
        .map(|trail_head| score(trail_head, shape, map.as_bytes()))
        .sum()
}

fn score(trail_head: (usize, char), shape: usize, map: &[u8]) -> u32 {
    let mut count = 0;
    let mut seen = HashSet::new();
    let mut stack = vec![trail_head];
    while let Some((idx, c)) = stack.pop() {
        seen.insert(idx);

        if c == '9' {
            count += 1;
            continue;
        }

        stack.extend(next_nodes(idx, shape, map, &seen));
    }
    count
}

pub fn rate_trail_heads(input: &str) -> u32 {
    let shape = input.find('\n').unwrap();
    let map: String = input.chars().filter(|&c| c != '\n').collect();
    map.char_indices()
        .filter(|(_, c)| *c == '0')
        .map(|trail_head| rate(trail_head, shape, map.as_bytes()))
        .sum()
}

fn rate(trail_head: (usize, char), shape: usize, map: &[u8]) -> u32 {
    let mut count = 0;
    let mut seen = HashSet::new();
    let mut stack = vec![trail_head];
    while let Some((idx, c)) = stack.pop() {
        seen.insert(idx);

        if c == '9' {
            count += 1;
            continue;
        }

        stack.extend(next_nodes(idx, shape, map, &HashSet::new()));
    }
    count
}

fn next_nodes(idx: usize, shape: usize, map: &[u8], seen: &HashSet<usize>) -> Vec<(usize, char)> {
    let (row, column) = (idx / shape, idx % shape);
    let mut nodes = vec![];

    // Left
    if column != 0 && map[idx] + 1 == map[idx - 1] && !seen.contains(&(idx - 1)) {
        nodes.push((idx - 1, map[idx - 1] as char));
    }

    // Right
    if column < shape - 1 && map[idx] + 1 == map[idx + 1] && !seen.contains(&(idx + 1)) {
        nodes.push((idx + 1, map[idx + 1] as char));
    }

    // Up
    if row > 0 && map[idx] + 1 == map[idx - shape] && !seen.contains(&(idx - shape)) {
        nodes.push((idx - shape, map[idx - shape] as char));
    }

    // Down
    if row < shape - 1 && map[idx] + 1 == map[idx + shape] && !seen.contains(&(idx + shape)) {
        nodes.push((idx + shape, map[idx + shape] as char));
    }

    nodes
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2024 - Day 10) --- {}, {}",
        score_trail_heads(input),
        rate_trail_heads(input)
    );
}
