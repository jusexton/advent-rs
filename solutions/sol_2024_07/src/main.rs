fn concat_numbers(a: u64, b: u64) -> u64 {
    let mut digit_count = 0;
    let mut tmp = b;
    while tmp > 0 {
        tmp /= 10;
        digit_count += 1;
    }

    a * (10_u64.pow(digit_count)) + b
}

fn calibration_exists(total: u64, numbers: Vec<u64>, operations: Vec<fn(u64, u64) -> u64>) -> bool {
    fn inner(
        idx: usize,
        current: u64,
        target: u64,
        numbers: &[u64],
        operations: &[fn(u64, u64) -> u64],
    ) -> bool {
        if idx >= numbers.len() {
            return current == target;
        }

        operations.iter().any(|op| {
            inner(
                idx + 1,
                op(current, numbers[idx]),
                target,
                numbers,
                operations,
            )
        })
    }

    inner(1, numbers[0], total, &numbers, &operations)
}

pub fn sum_calibrations(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (total, numbers) = line.split_once(": ").unwrap();
            let total = total.parse::<u64>().unwrap();
            let numbers: Vec<u64> = numbers.split(' ').map(|n| n.parse().unwrap()).collect();
            match calibration_exists(total, numbers, vec![|a, b| a + b, |a, b| a * b]) {
                true => Some(total),
                false => None,
            }
        })
        .sum()
}

pub fn sum_calibrations_with_concatenation(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (total, numbers) = line.split_once(": ").unwrap();
            let total = total.parse::<u64>().unwrap();
            let numbers: Vec<u64> = numbers.split(' ').map(|n| n.parse().unwrap()).collect();
            match calibration_exists(
                total,
                numbers,
                vec![|a, b| a + b, |a, b| a * b, |a, b| concat_numbers(a, b)],
            ) {
                true => Some(total),
                false => None,
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2024 - Day 7) --- {}, {}",
        sum_calibrations(input),
        sum_calibrations_with_concatenation(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concatenates_two_numbers() {
        assert_eq!(123456, concat_numbers(123, 456))
    }
}
