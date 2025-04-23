#![feature(int_from_ascii)]

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct SchematicNumber<'a> {
    idx: usize,
    bytes: &'a [u8],
}

impl SchematicNumber<'_> {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

#[derive(Debug)]
struct Schematics<'a> {
    numbers: Vec<SchematicNumber<'a>>,
    symbol_map: HashMap<usize, u8>,
    dimensions: (usize, usize),
}

fn parse_schematics(bytes: &[u8]) -> Schematics<'_> {
    let mut numbers = vec![];
    let mut symbol_map = HashMap::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'.' => i += 1,
            b if b.is_ascii_digit() => {
                let mut j = i + 1;
                while bytes[j].is_ascii_digit() {
                    j += 1;
                }
                numbers.push(SchematicNumber {
                    idx: i,
                    bytes: &bytes[i..j],
                });
                i += j - i
            }
            _ => {
                symbol_map.insert(i, bytes[i]);
                i += 1
            }
        };
    }

    Schematics {
        numbers,
        symbol_map,
        // Hard coded for now. Could be calculated
        dimensions: (140, 140),
    }
}

fn generate_adjacent(number: &SchematicNumber, shape: (usize, usize)) -> Vec<usize> {
    let mut adjacent = Vec::with_capacity((number.len() * 2) + 6);

    let start_idx = number.idx;
    let (row, column) = (start_idx / shape.0, start_idx % shape.1);
    if column != 0 {
        let iter: &[usize] = match row {
            0 => &[start_idx - 1, start_idx + shape.1 - 1],
            r if r == shape.1 => &[start_idx - 1, start_idx - shape.1 - 1],
            _ => &[
                start_idx - 1,
                start_idx + shape.1 - 1,
                start_idx - shape.1 - 1,
            ],
        };
        adjacent.extend(iter);
    }

    for i in start_idx..start_idx + number.len() {
        match row {
            0 => {
                adjacent.push(i + shape.1);
            }
            r if r == shape.0 => {
                adjacent.push(i - shape.1);
            }
            _ => {
                adjacent.push(i + shape.1);
                adjacent.push(i - shape.1);
            }
        };
    }

    let end_idx = number.idx + number.len() - 1;
    let (row, column) = (end_idx / shape.0, end_idx % shape.1);
    if column != shape.1 {
        let iter: &[usize] = match row {
            0 => &[end_idx + 1, end_idx + shape.1 + 1],
            r if r == shape.1 => &[end_idx + 1, end_idx - shape.1 + 1],
            _ => &[end_idx + 1, end_idx + shape.1 + 1, end_idx - shape.1 + 1],
        };
        adjacent.extend(iter);
    }

    adjacent
}

fn sum_engine_parts(input: &str) -> u32 {
    let bytes: Vec<_> = input.bytes().filter(|b| *b != b'\n').collect();
    let Schematics {
        symbol_map,
        numbers,
        dimensions,
    } = parse_schematics(&bytes);

    let mut sum = 0;
    for number in numbers {
        let adjacent = generate_adjacent(&number, dimensions);
        if adjacent.iter().any(|adj| symbol_map.contains_key(adj)) {
            let part_number = u32::from_ascii_radix(number.bytes, 10).unwrap();
            sum += part_number;
        }
    }
    sum
}

fn sum_gear_ratios(input: &str) -> u32 {
    let bytes: Vec<_> = input.bytes().filter(|b| *b != b'\n').collect();
    let Schematics {
        symbol_map,
        numbers,
        dimensions,
    } = parse_schematics(&bytes);

    let potential_locations: HashSet<_> = symbol_map
        .into_iter()
        .filter(|t| t.1 == b'*')
        .map(|t| t.0)
        .collect();

    let mut potential_gears = HashMap::new();
    for number in numbers {
        for adj in generate_adjacent(&number, dimensions) {
            if potential_locations.contains(&adj) {
                let part_numbers = u32::from_ascii_radix(number.bytes, 10).unwrap();
                potential_gears
                    .entry(adj)
                    .or_insert(vec![])
                    .push(part_numbers);
            }
        }
    }

    potential_gears
        .iter()
        .filter(|e| e.1.len() == 2)
        .map(|e| e.1.iter().product::<u32>())
        .sum()
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2023 - Day 3) --- {}, {}",
        sum_engine_parts(input),
        sum_gear_ratios(input)
    );
}
