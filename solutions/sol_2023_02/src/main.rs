use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Game {
    id: String,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl FromStr for Game {
    // Given the nature of the input, we always expect parsing to be successful.
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, game) = s.split_once(": ").unwrap();
        let mut max_red = u32::MIN;
        let mut max_green = u32::MIN;
        let mut max_blue = u32::MIN;
        let subsets = game.split("; ");
        for subset in subsets {
            let cubes = subset.split(", ");
            for cube in cubes {
                let (count, color) = cube.split_once(' ').unwrap();
                let count = count.parse().unwrap();
                match color {
                    "red" => max_red = max_red.max(count),
                    "green" => max_green = max_green.max(count),
                    "blue" => max_blue = max_blue.max(count),
                    _ => unreachable!(),
                }
            }
        }

        let (_, id) = id.split_once(' ').unwrap();
        Ok(Game {
            id: id.to_string(),
            max_red,
            max_blue,
            max_green,
        })
    }
}

fn possible_game_id_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter(|g| g.max_red <= 12 && g.max_green <= 13 && g.max_blue <= 14)
        .map(|g| g.id.parse::<u32>().unwrap())
        .sum()
}

fn minimum_present_cubes_multiplied(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .map(|g| g.max_blue * g.max_green * g.max_red)
        .sum()
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2023 - Day 2) --- {}, {}",
        possible_game_id_sum(input),
        minimum_present_cubes_multiplied(input)
    );
}
