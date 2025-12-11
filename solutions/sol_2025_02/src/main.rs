fn find_invalid_ids(range: (usize, usize)) -> Vec<usize> {
    let mut invalid = Vec::new();
    for id in range.0..=range.1 {
        let id_str = id.to_string();
        let id_bytes = id_str.as_bytes();
        if id_bytes.len() % 2 == 0 {
            let mid = id_bytes.len() / 2;
            if id_bytes[0..mid] == id_bytes[mid..] {
                invalid.push(id)
            }
        }
    }

    invalid
}

fn find_invalid_ids_v2(range: (usize, usize)) -> Vec<usize> {
    let mut invalid = Vec::new();
    for id in range.0..=range.1 {
        let id_str = id.to_string();
        let id_len = id_str.len();
        let id_bytes = id_str.as_bytes();

        for pattern_len in 1..id_len {
            if id_len % pattern_len == 0 {
                let pattern = &id_bytes[0..pattern_len];
                let rep_count = id_len / pattern_len;
                let test_seq = pattern.repeat(rep_count);
                if test_seq == id_bytes {
                    invalid.push(id);
                    break;
                }
            }
        }
    }

    invalid
}

fn sum_invalid_ids<F>(input: &[u8], mut find_invalid_ids: F) -> usize
where
    F: FnMut((usize, usize)) -> Vec<usize>,
{
    let mut sum = 0;
    let ranges = input.split(|b| b == &b',');
    for id_range in ranges {
        let mut iter = id_range
            .split(|b| b == &b'-')
            .map(|n| atoi::atoi::<usize>(n).unwrap());
        let range = (iter.next().unwrap(), iter.next().unwrap());
        sum += find_invalid_ids(range).into_iter().sum::<usize>();
    }
    sum
}

fn main() {
    let input = include_bytes!("../data.txt");
    println!(
        "(2025 - Day 2) --- {}, {}",
        sum_invalid_ids(input, find_invalid_ids),
        sum_invalid_ids(input, find_invalid_ids_v2)
    );
}
