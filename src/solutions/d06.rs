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

    fn trace(&self) -> char {
        match self {
            Direction::Up | Direction::Down => '|',
            Direction::Left | Direction::Right => '-',
        }
    }
}

enum Position {
    InBounds(usize),
    OutOfBounds,
}

fn find_next_position(idx: usize, shape: usize, direction: &Direction) -> Position {
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
    while let Position::InBounds(next_idx) = find_next_position(idx, shape, &direction) {
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

pub fn count_loop_possibilities(input: &str) -> u32 {
    let shape = input.find("\n").unwrap();
    let mut map: Vec<_> = input.chars().filter(|c| *c != '\n').collect();
    let mut idx = map.iter().position(|&c| c == '^').unwrap();
    let mut direction = Direction::Up;

    map[idx] = '.';
    let mut count = 1;
    while let Position::InBounds(next_idx) = find_next_position(idx, shape, &direction) {
        match (map[idx], map[next_idx]) {
            (_, '#') => direction = direction.cycle(),
            ('.', _) => {
                map[idx] = direction.trace();
                count += 1;
                idx = next_idx;
            }
            _ => idx = next_idx,
        }
    }

    count
}
