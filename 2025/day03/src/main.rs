use std::fs;

fn main() {
    let input = fs::read_to_string("./day03/input.txt").unwrap();

    println!("Day03 part 1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10))
                .collect::<Option<Vec<_>>>()
        })
        .map(|joltages| max_joltage(&joltages))
        .sum::<usize>()
}

fn max_joltage(battery_joltages: &[u32]) -> usize {
    assert!(battery_joltages.len() >= 2);
    let mut max_joltage = 0;
    let mut left = 0;

    for right in 1..battery_joltages.len() {
        let curr_digit = battery_joltages[right];
        max_joltage = max_joltage.max(battery_joltages[left] * 10 + curr_digit);

        if curr_digit > battery_joltages[left] && right < battery_joltages.len() - 1 {
            left = right
        }
    }

    max_joltage as usize
}
