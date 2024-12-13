#![feature(let_chains)]
#![feature(array_windows)]

mod solutions;
mod util;
use std::{
    fs,
    ops::RangeInclusive,
    time::{Duration, Instant},
};

use anyhow::{anyhow, Context, Result};

const DAY_RANGE: RangeInclusive<u8> = 1..=12;
const GENERAL_ERROR_MSG: &str = "Providing no value will run all solutions. To run a specific day provide the valid day as an argument. All other scenarios will be treated as errors.";

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => run_all(),
        2 => run_by_day(parse_args(args)?),
        _ => Err(anyhow::anyhow!(GENERAL_ERROR_MSG)),
    }
}

fn parse_args(args: Vec<String>) -> Result<u8> {
    match args[1].parse::<u8>() {
        Ok(n) if DAY_RANGE.contains(&n) => Ok(n),
        _ => Err(anyhow!(
            "Ensure provided day is inclusively within the range (1-25)"
        )),
    }
}

fn run_and_display<TOne, TTwo>(
    day: u8,
    input: &str,
    part_one: fn(&str) -> TOne,
    part_two: fn(&str) -> TTwo,
) where
    TOne: std::fmt::Display,
    TTwo: std::fmt::Display,
{
    run_and_display_with_suffix(day, input, part_one, part_two, "\n");
}

fn run_and_display_with_suffix<TOne, TTwo>(
    day: u8,
    input: &str,
    part_one: fn(&str) -> TOne,
    part_two: fn(&str) -> TTwo,
    suffix: &str,
) where
    TOne: std::fmt::Display,
    TTwo: std::fmt::Display,
{
    let (result_one, duration_one) = time_and_run(input, part_one);
    let (result_two, duration_two) = time_and_run(input, part_two);
    println!(
        "({} ms) Day {day} ===============",
        duration_one.as_millis() + duration_two.as_millis()
    );
    println!("({} ms) Part One: {result_one}", duration_one.as_millis());
    println!("({} ms) Part Two: {result_two}", duration_two.as_millis());
    print!("{suffix}")
}

fn time_and_run<T>(input: &str, f: fn(&str) -> T) -> (T, Duration)
where
    T: std::fmt::Display,
{
    let start = Instant::now();
    let result = f(input);
    let elapsed = start.elapsed();
    (result, elapsed)
}

fn run_by_day(day: u8) -> Result<()> {
    let path = format!("data/d{:02}.txt", day);
    let input = &fs::read_to_string(&path).context(format!("Failed to open {path}"))?;
    match day {
        1 => run_and_display(
            1,
            input,
            solutions::d01::list_distance,
            solutions::d01::list_similarity,
        ),
        2 => run_and_display(
            2,
            input,
            solutions::d02::count_safe_reports,
            solutions::d02::count_safe_reports_with_dampener,
        ),
        3 => run_and_display(
            3,
            input,
            solutions::d03::process_memory,
            solutions::d03::process_memory_with_conditionals,
        ),
        4 => run_and_display(
            4,
            input,
            solutions::d04::count_xmas,
            solutions::d04::count_x_mas,
        ),
        5 => run_and_display(
            5,
            input,
            solutions::d05::sum_valid_print_queue,
            solutions::d05::sum_corrected_print_queue,
        ),
        6 => run_and_display(
            6,
            input,
            solutions::d06::count_patrol_locations,
            solutions::incomplete,
        ),
        7 => run_and_display(
            7,
            input,
            solutions::d07::sum_calibrations,
            solutions::d07::sum_calibrations_with_concatenation,
        ),
        8 => run_and_display(
            8,
            input,
            solutions::d08::count_anti_nodes,
            solutions::d08::count_resonant_anti_nodes,
        ),
        9 => run_and_display(
            9,
            input,
            solutions::d09::defrag_file_bits_and_checksum,
            solutions::d09::defrag_file_chunk_and_checksum,
        ),
        10 => run_and_display(
            10,
            input,
            solutions::d10::score_trail_heads,
            solutions::d10::rate_trail_heads,
        ),
        11 => run_and_display(
            11,
            input,
            |input| solutions::d11::stone_count_after_blinking(input, 25),
            |input| solutions::d11::stone_count_after_blinking(input, 75),
        ),
        12 => run_and_display_with_suffix(
            12,
            input,
            solutions::d12::garden_plot_price,
            solutions::incomplete,
            "",
        ),
        _ => {}
    }

    Ok(())
}

fn run_all() -> Result<()> {
    DAY_RANGE.clone().try_for_each(run_by_day)
}
