fn is_safe(numbers: &[u32]) -> bool {
    if numbers.len() <= 1 {
        return true;
    }

    let diffs: Vec<_> = numbers
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .collect();
    let increasing = diffs[0] > 0;
    for diff in diffs {
        if (increasing && diff <= 0) || (!increasing && diff >= 0) {
            return false;
        }
        if diff.abs_diff(0) < 1 || diff.abs_diff(0) > 3 {
            return false;
        }
    }

    true
}

pub fn count_safe_reports(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|l| l.parse().ok())
                .collect::<Vec<_>>()
        })
        .filter(|numbers| is_safe(numbers))
        .count()
}

fn is_safe_with_dampener(numbers: &[u32]) -> bool {
    if is_safe(numbers) {
        return true;
    }

    (0..numbers.len()).any(|i| {
        let mut dampened = Vec::from(numbers);
        dampened.remove(i);
        is_safe(&dampened)
    })
}

pub fn count_safe_reports_with_dampener(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|l| l.parse().ok())
                .collect::<Vec<_>>()
        })
        .filter(|l| is_safe_with_dampener(l))
        .count()
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2024 - Day 2) --- {}, {}",
        count_safe_reports(input),
        count_safe_reports_with_dampener(input)
    );
}
