use std::str::FromStr;

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

pub fn stone_count_after_blinking(input: &str) -> u32 {
    let mut numbers: Vec<u64> = parse_numbers(input);
    for _ in 0..6 {
        let mut new_numbers = Vec::with_capacity(100_000);
        for number in numbers {
            let digits = get_digits(number);
            if number == 0 {
                new_numbers.push(1);
            } else if digits.len() % 2 == 0 {
                let (left, right) = split_digits(digits);
                new_numbers.push(left);
                new_numbers.push(right);
            } else {
                new_numbers.push(number * 2024);
            }
        }
        numbers = new_numbers
    }
    numbers.len() as u32
}
