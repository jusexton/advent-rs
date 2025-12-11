const TOTAL_TICKS: i32 = 100;

struct SolvedPassword {
    land_on_zero: u32,
    touch_zero: u32,
}

impl SolvedPassword {
    fn new() -> Self {
        Self {
            land_on_zero: 0,
            touch_zero: 0,
        }
    }
}

fn solve_password(input: &[u8]) -> SolvedPassword {
    let mut dial: i32 = 50;
    let mut result = SolvedPassword::new();
    for line in input.split(|b| b == &b'\n') {
        let mut distance = atoi::atoi::<i32>(&line[1..]).unwrap();
        if line[0] == b'L' {
            distance *= -1;
        }

        let new_dial = dial.wrapping_add(distance);
        result.touch_zero +=
            (new_dial <= 0 && dial != 0) as u32 + (new_dial.signum() * new_dial / 100) as u32;

        dial = new_dial.rem_euclid(TOTAL_TICKS);
        result.land_on_zero += (dial == 0) as u32;
    }
    result
}

fn main() {
    let input = include_bytes!("../data.txt");
    let res = solve_password(input);
    println!(
        "(2025 - Day 1) --- {}, {}",
        res.land_on_zero, res.touch_zero
    );
}
