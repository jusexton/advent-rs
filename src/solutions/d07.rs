use std::collections::VecDeque;

fn concat_numbers(a: u64, b: u64) -> u64 {
    let mut digit_count = 0;
    let mut tmp = b;
    while tmp > 0 {
        tmp /= 10;
        digit_count += 1;
    }

    a * (10_u64.pow(digit_count)) + b
}

fn generate_calibrations(numbers: Vec<u64>, operations: &[fn(u64, u64) -> u64]) -> Vec<u64> {
    fn inner(
        mut numbers: VecDeque<u64>,
        results: &mut Vec<u64>,
        operations: &[fn(u64, u64) -> u64],
    ) {
        if numbers.len() < 2 {
            results.push(numbers[0]);
            return;
        }

        while let Some(val_one) = numbers.pop_front()
            && let Some(val_two) = numbers.pop_front()
        {
            for op in operations {
                let mut n1 = numbers.clone();
                n1.push_front(op(val_one, val_two));
                inner(n1, results, operations);
            }
        }
    }

    let mut results = vec![];
    inner(VecDeque::from(numbers), &mut results, operations);
    results
}

fn calibration_exists(total: u64, numbers: Vec<u64>, operations: Vec<fn(u64, u64) -> u64>) -> bool {
    generate_calibrations(numbers, &operations).contains(&total)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_all_calibrations() {
        assert_eq!(
            vec![6, 5, 9, 6],
            generate_calibrations(vec![1, 2, 3], &[|a, b| a * b, |a, b| a + b])
        )
    }
    #[test]
    fn concatenates_two_numbers() {
        assert_eq!(123456, concat_numbers(123, 456))
    }
}
