use std::{collections::HashMap, str::FromStr};

fn parse_numbers<T>(input: &str) -> Vec<T>
where
    T: FromStr,
{
    input
        .split_whitespace()
        .flat_map(|n| n.parse::<T>().ok())
        .collect()
}

fn get_digits(mut number: u64) -> Vec<i8> {
    let mut digits = vec![];
    while number > 0 {
        digits.push((number % 10) as i8);
        number /= 10;
    }
    digits
}

fn split_digits(digits: Vec<i8>) -> (u64, u64) {
    fn create_number(numbers: &[i8]) -> u64 {
        let mut number = 0;
        for (idx, &digit) in numbers.iter().enumerate() {
            number += digit as u64 * 10u64.pow(idx as u32);
        }
        number
    }

    let mid = digits.len() / 2;
    (
        create_number(&digits[0..mid]),
        create_number(&digits[mid..]),
    )
}

fn stone_count(number: u64, depth: usize) -> u64 {
    fn inner(number: u64, depth: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
        if depth == 0 {
            return 1;
        }
        if let Some(&value) = cache.get(&(number, depth)) {
            return value;
        }

        let digits = get_digits(number);
        let result = match number {
            0 => inner(1, depth - 1, cache),
            _ if digits.len() % 2 == 0 => {
                let (left, right) = split_digits(digits);
                inner(left, depth - 1, cache) + inner(right, depth - 1, cache)
            }
            _ => inner(2024 * number, depth - 1, cache),
        };
        cache.insert((number, depth), result);
        result
    }

    inner(number, depth, &mut HashMap::new())
}

pub fn stone_count_after_blinking(input: &str, count: usize) -> u64 {
    parse_numbers(input)
        .iter()
        .map(|&number| stone_count(number, count))
        .sum()
}
