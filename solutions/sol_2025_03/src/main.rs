fn max_joltage(input: &[u8], battery_count: u8) -> u64 {
    let battery_count = battery_count as usize;
    let mut stack = Vec::new();
    let mut to_remove = input.len() - battery_count;

    for digit in input {
        while to_remove > 0
            && let Some(last) = stack.last()
            && last < digit
        {
            stack.pop();
            to_remove -= 1;
        }

        stack.push(*digit);
    }

    let max_joltage = &stack[0..battery_count];
    atoi::atoi(max_joltage).expect("expected max_joltage to be a valid u64")
}

fn total_joltage(input: &[u8], batteries: u8) -> u64 {
    input
        .split(|b| *b == b'\n')
        .map(|bank| max_joltage(bank, batteries))
        .sum()
}

fn main() {
    let input = include_bytes!("../data.txt");
    println!(
        "(2025 - Day 3) --- {}, {}",
        total_joltage(input, 2),
        total_joltage(input, 12)
    );
}
