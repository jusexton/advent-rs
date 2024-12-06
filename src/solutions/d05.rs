use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::util::NumberLineParser;

type PrintRules = HashMap<u32, (HashSet<u32>, HashSet<u32>)>;

fn build_rules(rule_str: &str) -> PrintRules {
    rule_str.lines().fold(HashMap::new(), |mut acc, line| {
        let page_numbers: Vec<u32> = line.split('|').filter_map(|v| v.parse().ok()).collect();

        let value = acc
            .entry(page_numbers[0])
            .or_insert((HashSet::new(), HashSet::new()));
        value.0.insert(page_numbers[1]);

        let value = acc
            .entry(page_numbers[1])
            .or_insert((HashSet::new(), HashSet::new()));
        value.1.insert(page_numbers[0]);

        acc
    })
}

fn compare_by_rules(a: &u32, b: &u32, rules: &PrintRules) -> Ordering {
    let (greater, less) = rules.get(a).unwrap();
    if greater.contains(b) {
        return Ordering::Greater;
    }
    if less.contains(b) {
        return Ordering::Less;
    }
    Ordering::Equal
}

fn is_valid_print_queue(numbers: &[u32], rules: &PrintRules) -> bool {
    numbers.is_sorted_by(|a, b| rules.get(a).map(|after| after.0.contains(b)).unwrap())
}

pub fn sum_valid_print_queue(input: &str) -> u32 {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let rules = build_rules(rules);
    NumberLineParser::split_comma(pages)
        .flatten()
        .filter(|line| is_valid_print_queue(line, &rules))
        .map(|line| line[line.len() / 2])
        .sum()
}

pub fn sum_corrected_print_queue(input: &str) -> u32 {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let rules = build_rules(rules);
    NumberLineParser::split_comma(pages)
        .flatten()
        .filter(|line| !is_valid_print_queue(line, &rules))
        .map(|mut line| {
            line.sort_unstable_by(|a, b| compare_by_rules(a, b, &rules));
            line[line.len() / 2]
        })
        .sum()
}
