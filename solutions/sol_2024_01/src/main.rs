use std::collections::HashMap;

fn list_distance(input: &str) -> u32 {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|d| d.parse().ok())
            .collect();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn list_similarity(input: &str) -> u32 {
    let mut left = vec![];
    let mut right_freq = HashMap::new();
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|d| d.parse().ok())
            .collect();

        left.push(numbers[0]);
        *right_freq.entry(numbers[1]).or_insert(0) += 1;
    }

    left.into_iter()
        .map(|v| v * right_freq.get(&v).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2024 - Day 1) --- {}, {}",
        list_distance(input),
        list_similarity(input)
    );
}
