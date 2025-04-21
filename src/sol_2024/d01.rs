use std::collections::HashMap;

use crate::util::NumberLineParser;

pub fn list_distance(input: &str) -> u32 {
    let mut left = vec![];
    let mut right = vec![];
    for line in NumberLineParser::split_whitespace(input).flatten() {
        left.push(line[0]);
        right.push(line[1]);
    }

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

pub fn list_similarity(input: &str) -> u32 {
    let mut left = vec![];
    let mut right_freq = HashMap::new();
    for line in NumberLineParser::split_whitespace(input).flatten() {
        left.push(line[0]);
        *right_freq.entry(line[1]).or_insert(0) += 1;
    }

    left.into_iter()
        .map(|v| v * right_freq.get(&v).unwrap_or(&0))
        .sum()
}
