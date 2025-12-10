use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("./day03/input.txt").unwrap();

    println!("Day03 part 1: {}", part_1(&input));
    println!("Day03 part 2: {}", part_2(&input))
}

fn part_2(input: &str) -> usize {
    fn max_joltage(joltages: &[u32]) -> usize {
        assert!(joltages.len() >= 12);
        let mut stack = VecDeque::<u32>::new();
        let len = joltages.len();

        for (idx, &joltage) in joltages.iter().enumerate() {
            let remaining = len - idx - 1;

            while remaining >= 12 - stack.len()&& stack.back().is_some_and(|&back| back < joltage)
            {
                let _ = stack.pop_back();
            }

            if stack.len() < 12 {
                stack.push_back(joltage);
            }
        }

        assert_eq!(stack.len(), 12);
        stack
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, digit)| {
                acc + (digit as usize * 10usize.pow(idx as u32))
            })
    }

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

fn part_1(input: &str) -> usize {
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
