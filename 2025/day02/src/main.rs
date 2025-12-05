use std::{fs, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("./day02/input.txt").unwrap();

    let ranges = parse_range(&input);

    println!("Day02 part 1: {}", part_1(&ranges));
    println!("Day02 part 2: {}", part_2(&ranges));
}

fn part_1(ranges: &[RangeInclusive<usize>]) -> usize {
    ranges.iter().fold(0usize, |acc, range| {
        acc + range
            .clone()
            .filter(|&id| is_invalid_id_1(id))
            .sum::<usize>()
    })
}

fn part_2(ranges: &[RangeInclusive<usize>]) -> usize {
    ranges.iter().fold(0usize, |acc, range| {
        acc + range
            .clone()
            .filter(|&id| is_invalid_id_2(id))
            .sum::<usize>()
    })
}

fn is_invalid_id(id: usize, allowed_group_sizes: impl Iterator<Item = usize>) -> bool {
    let digits = digits(id);

    'outer: for group_size in allowed_group_sizes {
        if !digits.is_multiple_of(group_size) {
            continue;
        }

        let group_count = digits / group_size;

        for group in 1..group_count {
            for group_offset in 0..group_size {
                let curr_idx = group * group_size + group_offset;
                let prev_idx = (group - 1) * group_size + group_offset;

                if nth_digit_front(id, digits, curr_idx as u32)
                    != nth_digit_front(id, digits, prev_idx as u32)
                {
                    continue 'outer;
                }
            }
        }

        return true;
    }

    false
}

fn is_invalid_id_1(id: usize) -> bool {
    let digits = digits(id);
    if digits % 2 == 1 {
        return false;
    }
    let group_size = digits / 2;

    is_invalid_id(id, [group_size].into_iter())
}

fn is_invalid_id_2(id: usize) -> bool {
    let digits = digits(id);
    if digits < 2 {
        return false;
    }

    is_invalid_id(
        id,
        (1..=digits / 2).filter(|&group_size| digits.is_multiple_of(group_size)),
    )
}

fn parse_range(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .split(",")
        .map(|range| {
            let (first_str, second_str) = range.split_once("-").unwrap();
            let (first, second) = (first_str.parse().unwrap(), second_str.parse().unwrap());

            first..=second
        })
        .collect()
}

fn nth_digit_front(num: usize, digits: usize, n: u32) -> usize {
    (num / 10usize.pow(digits as u32 - 1 - n)) % 10
}

fn digits(num: usize) -> usize {
    (num.checked_ilog10().unwrap_or(0) + 1) as usize
}
