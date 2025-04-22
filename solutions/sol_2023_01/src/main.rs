use std::sync::LazyLock;

use regex::Regex;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)").unwrap()
});

fn find_calibration_value(s: &str) -> u32 {
    let left = s.chars().find(|c| c.is_ascii_digit()).unwrap();
    let right = s.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
    format!("{}{}", left, right).parse().unwrap()
}

fn calibration_values(input: &str) -> u32 {
    input.lines().map(find_calibration_value).sum()
}

fn to_digit(s: &str) -> &str {
    if s.len() == 1 {
        return s;
    }

    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => panic!("invalid digit"),
    }
}

fn find_spelled_calibration_value(s: &str) -> u32 {
    let matches: Vec<_> = REGEX.find_iter(s).collect();
    let left = to_digit(matches[0].as_str());
    let right = to_digit(matches[matches.len() - 1].as_str());
    format!("{left}{right}").parse().unwrap()
}

#[allow(dead_code)]
fn spelled_calibration_values(input: &str) -> u32 {
    input.lines().map(find_spelled_calibration_value).sum()
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2023 - Day 1) --- {}, INCOMPLETE",
        calibration_values(input)
    );
}
