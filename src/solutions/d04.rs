const SHAPE: usize = 140;

pub fn count_xmas(input: &str) -> u32 {
    fn is_xmas(m: char, a: char, s: char) -> bool {
        m == 'M' && a == 'A' && s == 'S'
    }

    let mut count = 0;
    let b: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    for (idx, _) in b.iter().enumerate().filter(|(_, &ch)| ch == 'X') {
        let (row, column) = (idx / SHAPE, idx % SHAPE);

        // Horizontal Right
        if column <= SHAPE - 4 && is_xmas(b[idx + 1], b[idx + 2], b[idx + 3]) {
            count += 1;
        }
        // Horizontal Left
        if column >= 3 && is_xmas(b[idx - 1], b[idx - 2], b[idx - 3]) {
            count += 1;
        }

        // Vertical Up
        if row >= 3 && is_xmas(b[idx - SHAPE], b[idx - SHAPE * 2], b[idx - SHAPE * 3]) {
            count += 1;
        }
        // Vertical Down
        if row <= SHAPE - 4 && is_xmas(b[idx + SHAPE], b[idx + SHAPE * 2], b[idx + SHAPE * 3]) {
            count += 1;
        }

        // Horizontal Up Right
        if row >= 3
            && column <= SHAPE - 4
            && is_xmas(
                b[idx - SHAPE + 1],
                b[idx - SHAPE * 2 + 2],
                b[idx - SHAPE * 3 + 3],
            )
        {
            count += 1;
        }
        // Horizontal Up Left
        if row >= 3
            && column >= 3
            && is_xmas(
                b[idx - SHAPE - 1],
                b[idx - SHAPE * 2 - 2],
                b[idx - SHAPE * 3 - 3],
            )
        {
            count += 1;
        }
        // Horizontal Down Right
        if row <= SHAPE - 4
            && column <= SHAPE - 4
            && is_xmas(
                b[idx + SHAPE + 1],
                b[idx + SHAPE * 2 + 2],
                b[idx + SHAPE * 3 + 3],
            )
        {
            count += 1;
        }
        // Horizontal Down Left
        if row <= SHAPE - 4
            && column >= 3
            && is_xmas(
                b[idx + SHAPE - 1],
                b[idx + SHAPE * 2 - 2],
                b[idx + SHAPE * 3 - 3],
            )
        {
            count += 1;
        }
    }
    count
}

pub fn count_x_mas(input: &str) -> u32 {
    let chars: Vec<_> = input.chars().filter(|c| *c != '\n').collect();
    let mut i = SHAPE * 2 + 2;
    let mut count = 0;
    while i < chars.len() {
        let column = i % SHAPE;
        if column == 0 {
            i += 2;
        }

        if chars[i - SHAPE - 1] == 'A' {
            let t_left = chars[i - SHAPE * 2 - 2];
            let b_right = chars[i];
            let t_right = chars[i - SHAPE * 2];
            let b_left = chars[i - 2];

            let diagonal_one = t_left == 'M' && b_right == 'S' || t_left == 'S' && b_right == 'M';
            let diagonal_two = b_left == 'M' && t_right == 'S' || b_left == 'S' && t_right == 'M';
            if diagonal_one && diagonal_two {
                count += 1;
            }
        }

        i += 1;
    }

    count
}
