#![feature(let_chains)]
#![feature(array_windows)]

mod solutions;
mod util;

use std::{fs, ops::RangeInclusive};

use anyhow::{anyhow, Context, Result};

const DAY_RANGE: RangeInclusive<u8> = 1..=8;
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

fn run_by_day(day: u8) -> Result<()> {
    let path = format!("data/d{:02}.txt", day);
    let input = &fs::read_to_string(&path).context(format!("Failed to open {path}"))?;
    match day {
        1 => {
            let distance = solutions::d01::list_distance(input);
            let similarity = solutions::d01::list_similarity(input);
            println!("== Day One ==\nDistance: {distance}\nSimilarity: {similarity}\n");
        }
        2 => {
            let safe_reports = solutions::d02::count_safe_reports(input);
            let safe_reports_with_dampener =
                solutions::d02::count_safe_reports_with_dampener(input);
            println!("== Day Two ==\nSafe Reports: {safe_reports}\nSafe Reports With Dampener: {safe_reports_with_dampener}\n");
        }
        3 => {
            let multiplications = solutions::d03::process_memory(input);
            let with_conditionals = solutions::d03::process_memory_with_conditionals(input);
            println!("== Day Three ==\nMultiplications: {multiplications}\nWith Conditionals: {with_conditionals}\n");
        }
        4 => {
            let xmas_count = solutions::d04::count_xmas(input);
            let x_mas_count = solutions::d04::count_x_mas(input);
            println!("== Day Four ==\nXmas Count: {xmas_count}\nX-Mas Count: {x_mas_count}\n");
        }
        5 => {
            let valid_sum = solutions::d05::sum_valid_print_queue(input);
            let corrected_sum = solutions::d05::sum_corrected_print_queue(input);
            println!("== Day Five ==\nValid Print Queue Sum: {valid_sum}\nCorrected Print Queue Sum: {corrected_sum}\n");
        }
        6 => {
            let patrol_locations = solutions::d06::count_patrol_locations(input);
            let loop_possibilities = solutions::d06::count_loop_possibilities(input);
            println!("== Day Six ==\nPatrol Locations: {patrol_locations}\nLoop Possibilities: {loop_possibilities}\n");
        }
        7 => {
            let summed_calibrations = solutions::d07::sum_calibrations(input);
            let summed_calibrations_with_concat =
                solutions::d07::sum_calibrations_with_concatenation(input);
            println!("== Day Seven ==\nSummed Calibrations: {summed_calibrations}\nSummed Calibrations With Concatenation: {summed_calibrations_with_concat}\n");
        }
        8 => {
            let node_count = solutions::d08::count_anti_nodes(input);
            let node_count_with_resonance = solutions::d08::count_resonant_anti_nodes(input);
            println!("== Day Eight ==\nAnti Node Count: {node_count}\nAnti Node Count with Resonance: {node_count_with_resonance}");
        }
        _ => {}
    }

    Ok(())
}

fn run_all() -> Result<()> {
    DAY_RANGE.clone().try_for_each(run_by_day)
}
