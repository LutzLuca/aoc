#![feature(array_windows)]
#![feature(iter_map_windows)]
use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("./day02/input.txt").unwrap();

    println!("Day02 part 1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|levels| {
            let mut levels = levels
                .split(' ')
                .map(|level| level.parse::<usize>().unwrap())
                .tuple_windows();

            levels
                .next()
                .and_then(|(a, b)| (a.abs_diff(b) <= 3).then_some(a.cmp(&b)))
                .map(|ord| levels.all(|(a, b)| a.cmp(&b) == ord && a.abs_diff(b) <= 3))
                .unwrap_or(false)
        })
        .count()
}
