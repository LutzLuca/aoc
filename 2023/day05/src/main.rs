use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("day05/input.txt").unwrap();
    part_1(&input);
}

fn part_1(input: &str) {
    let (seeds, maps) = input.split_once("\r\n").unwrap();
    let seeds: Vec<_> = seeds[(seeds.find(':').unwrap() + 1)..]
        .split_ascii_whitespace()
        .map(|val| val.parse::<usize>().unwrap())
        .collect();

    let result = maps
        .lines()
        .group_by(|line| line.starts_with(|ch: char| ch.is_ascii_digit()))
        .into_iter()
        .filter(|(matches, _)| *matches)
        .map(|(_, group)| {
            group.map(|range| {
                let (dest, src, len) = range
                    .split_ascii_whitespace()
                    .map(|digit| digit.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                (src..(src + len), dest)
            })
        })
        .fold(seeds, |mut acc, mut mapping_ranges| {
            acc.iter_mut().for_each(|seed| {
                if let Some((range, dest_start)) =
                    mapping_ranges.find(|(range, _)| range.contains(seed))
                {
                    *seed = (*seed - range.start) + dest_start
                }
            });
            acc
        })
        .into_iter()
        .min()
        .unwrap();
    println!("Day05 Part 1: {result}")
}
