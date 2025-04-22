use regex::Regex;

pub fn process_memory(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|m| {
            let (_, [a, b]) = m.extract();
            a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
        })
        .sum()
}

pub fn process_memory_with_conditionals(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    re.captures_iter(input)
        .map(|m| match m.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => {
                if enabled {
                    let a = m.get(1).unwrap().as_str();
                    let b = m.get(2).unwrap().as_str();
                    a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
                } else {
                    0
                }
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2024 - Day 3) --- {}, {}",
        process_memory(input),
        process_memory_with_conditionals(input)
    );
}
