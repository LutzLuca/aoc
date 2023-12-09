#![feature(iter_map_windows)]
use std::fs;

fn main() {
    let input = fs::read_to_string("day09/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let result: isize = input
        .split("\r\n")
        .map(|line| {
            let mut res = 0;
            let mut seq: Vec<_> = line
                .split(' ')
                .map(|val| val.parse::<isize>().unwrap())
                .collect();

            while seq.iter().any(|&num| num != 0) {
                res += *seq.last().unwrap();
                seq = seq.into_iter().map_windows(|[x, y]| y - x).collect();
            }
            res
        })
        .sum();

    println!("Day09 Part 1: {result}");
}

fn part_2(input: &str) {
    let result: isize = input
        .split("\r\n")
        .map(|line| {
            let mut nums = vec![];
            let mut seq: Vec<_> = line
                .split(' ')
                .map(|val| val.parse::<isize>().unwrap())
                .collect();

            while seq.iter().any(|&num| num != 0) {
                nums.push(*seq.first().unwrap());
                seq = seq.into_iter().map_windows(|[x, y]| y - x).collect();
            }
            nums.into_iter().rev().fold(0, |acc, fst| fst - acc)
        })
        .sum();

    println!("Day09 Part 2: {result}");
}
