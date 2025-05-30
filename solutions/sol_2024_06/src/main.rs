use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn cycle(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

enum Position {
    InBounds(usize),
    OutOfBounds,
}

fn find_next_position(idx: usize, shape: usize, direction: Direction) -> Position {
    let (row, column) = (idx / shape, idx % shape);
    match direction {
        Direction::Up => match row == 0 {
            true => Position::OutOfBounds,
            false => Position::InBounds(idx - shape),
        },
        Direction::Right => match column == shape - 1 {
            true => Position::OutOfBounds,
            false => Position::InBounds(idx + 1),
        },
        Direction::Down => match row == shape - 1 {
            true => Position::OutOfBounds,
            false => Position::InBounds(idx + shape),
        },
        Direction::Left => match column == 0 {
            true => Position::OutOfBounds,
            false => Position::InBounds(idx - 1),
        },
    }
}

pub fn count_patrol_locations(input: &str) -> u32 {
    let shape = input.find("\n").unwrap();
    let mut map: Vec<_> = input.chars().filter(|c| *c != '\n').collect();
    let mut idx = map.iter().position(|&c| c == '^').unwrap();
    let mut direction = Direction::Up;

    map[idx] = '.';
    let mut count = 1;
    while let Position::InBounds(next_idx) = find_next_position(idx, shape, direction) {
        match (map[idx], map[next_idx]) {
            (_, '#') => direction = direction.cycle(),
            ('.', _) => {
                map[idx] = 'X';
                count += 1;
                idx = next_idx;
            }
            _ => idx = next_idx,
        }
    }

    count
}

#[allow(dead_code)]
fn obstacle_simulation(
    original_idx: usize,
    original_next_idx: usize,
    shape: usize,
    original_dir: Direction,
    map: &[char],
) -> bool {
    let mut idx = original_idx;
    let mut direction = original_dir.cycle();
    let mut visited = HashSet::new();
    while let Position::InBounds(next_idx) = find_next_position(idx, shape, direction) {
        if map[next_idx] == '#' || next_idx == original_next_idx {
            direction = direction.cycle();
        } else {
            visited.insert((idx, direction));
            idx = next_idx;
        }

        if visited.contains(&(idx, direction)) {
            return idx == original_idx;
        }
    }

    false
}

#[allow(dead_code)]
pub fn count_loop_possibilities(input: &str) -> u32 {
    let shape = input.find("\n").unwrap();
    let mut map: Vec<_> = input.chars().filter(|c| *c != '\n').collect();
    let mut idx = map.iter().position(|&c| c == '^').unwrap();
    let mut direction = Direction::Up;

    map[idx] = '.';
    let mut positions = HashSet::new();
    // Main path traversal
    while let Position::InBounds(next_idx) = find_next_position(idx, shape, direction) {
        match (map[idx], map[next_idx]) {
            (_, '#') => direction = direction.cycle(),
            ('.', _) => {
                // On every valid move, pretend there is an obstacle ahead and play out the movements.
                // If the cursor makes it exactly back to where it started, a loop has been identified.
                let loop_detected = obstacle_simulation(idx, next_idx, shape, direction, &map);
                if loop_detected {
                    positions.insert(next_idx);
                }

                idx = next_idx;
            }
            _ => idx = next_idx,
        }
    }

    positions.len() as u32
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2024 - Day 6) --- {}, INCOMPLETE",
        count_patrol_locations(input)
    );
}
