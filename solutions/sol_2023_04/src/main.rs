use std::collections::HashSet;

fn parse_numbers(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    fn collect_numbers(s: &str) -> HashSet<u32> {
        s.split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect()
    }

    let (_, numbers) = line.split_once(": ").unwrap();
    let (winning, owned) = numbers.split_once(" | ").unwrap();
    (collect_numbers(winning), collect_numbers(owned))
}

fn score_cards(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (winning, owned) = parse_numbers(line);
            let count = winning.intersection(&owned).count();
            match count {
                0 => 0,
                _ => 1 << (count - 1),
            }
        })
        .sum()
}

fn count_card_instances(input: &str) -> u32 {
    let win_counts: Vec<_> = input
        .lines()
        .map(|line| {
            let (winning, owned) = parse_numbers(line);
            winning.intersection(&owned).count()
        })
        .collect();

    let mut sum = win_counts.len();
    let mut cache = vec![0; win_counts.len()];
    for i in (0..win_counts.len()).rev() {
        let mut win_count = win_counts[i];
        let mut curr_card = i + 1;
        let mut card_count = 0;
        while curr_card < win_counts.len() && win_count > 0 {
            card_count += cache[curr_card] + 1;
            curr_card += 1;
            win_count -= 1;
        }
        cache[i] = card_count;
        sum += card_count;
    }
    sum as u32
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2023 - Day 4) --- {}, {}",
        score_cards(input),
        count_card_instances(input)
    );
}
