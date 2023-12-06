use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs;

fn main() {
    let input = fs::read_to_string("day06/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let (times, distances) = input.split_once("\r\n").unwrap();
    let times = times
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());

    let distances = distances
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());

    let result = times
        .zip(distances)
        .map(|(max_time, high_score)| {
            (1..max_time)
                .map(|t| t * (max_time - t))
                .filter(|&dist| dist > high_score)
                .count()
        })
        .reduce(|acc, ways| acc * ways)
        .unwrap();

    println!("Day06 Part 1: {result}")
}

fn part_2(input: &str) {
    let (times, dists) = input.split_once("\r\n").unwrap();
    let time = times
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let high_score = dists
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let result: usize = (1..=time)
        .into_par_iter()
        .map(|t| t * (time - t))
        .fold(
            || 0,
            |acc, score| if score < high_score { acc } else { acc + 1 },
        )
        .sum();

    println!("Day06 Part 2: {result}")
}
