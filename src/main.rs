#![feature(let_chains)]
#![feature(array_windows)]

mod solutions;
mod util;
use std::{fs, ops::RangeInclusive};

use anyhow::{anyhow, Context, Result};

const DAY_RANGE: RangeInclusive<u8> = 1..=9;
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

fn display_results<TOne, TTwo>(day: u8, part_one: TOne, part_two: TTwo)
where
    TOne: std::fmt::Display,
    TTwo: std::fmt::Display,
{
    display_results_with_suffix(day, part_one, part_two, "\n");
}

fn display_results_with_suffix<TOne, TTwo>(day: u8, part_one: TOne, part_two: TTwo, suffix: &str)
where
    TOne: std::fmt::Display,
    TTwo: std::fmt::Display,
{
    println!("==== Day {day} ====");
    println!("Part One: {part_one}");
    println!("Part Two: {part_two}");
    print!("{suffix}")
}

fn run_by_day(day: u8) -> Result<()> {
    let path = format!("data/d{:02}.txt", day);
    let input = &fs::read_to_string(&path).context(format!("Failed to open {path}"))?;
    match day {
        1 => display_results(
            1,
            solutions::d01::list_distance(input),
            solutions::d01::list_similarity(input),
        ),
        2 => display_results(
            2,
            solutions::d02::count_safe_reports(input),
            solutions::d02::count_safe_reports_with_dampener(input),
        ),
        3 => display_results(
            3,
            solutions::d03::process_memory(input),
            solutions::d03::process_memory_with_conditionals(input),
        ),
        4 => display_results(
            4,
            solutions::d04::count_xmas(input),
            solutions::d04::count_x_mas(input),
        ),
        5 => display_results(
            5,
            solutions::d05::sum_valid_print_queue(input),
            solutions::d05::sum_corrected_print_queue(input),
        ),
        6 => display_results(
            6,
            solutions::d06::count_patrol_locations(input),
            solutions::incomplete(),
        ),
        7 => display_results(
            7,
            solutions::d07::sum_calibrations(input),
            solutions::d07::sum_calibrations_with_concatenation(input),
        ),
        8 => display_results(
            8,
            solutions::d08::count_anti_nodes(input),
            solutions::d08::count_resonant_anti_nodes(input),
        ),
        9 => display_results_with_suffix(
            9,
            solutions::d09::defrag_file_bits_and_checksum(input),
            solutions::d09::defrag_file_chunk_and_checksum(input),
            "",
        ),
        _ => {}
    }

    Ok(())
}

fn run_all() -> Result<()> {
    DAY_RANGE.clone().try_for_each(run_by_day)
}
