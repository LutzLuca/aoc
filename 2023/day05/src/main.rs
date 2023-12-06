#![feature(iter_array_chunks)]
#![allow(clippy::single_range_in_vec_init)]
use itertools::Itertools;
use std::{cmp, fs, ops::Range, usize};

fn main() {
    let input = fs::read_to_string("day05/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn apply_mappings(
    curr: Range<usize>,
    mapping_ranges: &[(Range<usize>, usize)],
) -> Vec<Range<usize>> {
    // There has to be a cleaner way to for list concatenation
    if let Some((range, dest_start)) = mapping_ranges.first() {
        let mapped_ranges = if curr.end < range.start || curr.start >= range.end {
            apply_mappings(curr, &mapping_ranges[1..])
        } else if curr.start >= range.start && curr.end <= range.end {
            vec![(dest_start + curr.start - range.start)..(dest_start + curr.end - range.start)]
        } else if curr.start < range.start && curr.end > range.end {
            let mut ranges = apply_mappings(curr.start..range.start, &mapping_ranges[1..]);
            ranges.extend(
                apply_mappings(range.start..range.end, mapping_ranges)
                    .into_iter()
                    .chain(apply_mappings(range.end..curr.end, &mapping_ranges[1..])),
            );
            ranges
        } else if curr.start < range.start && curr.end <= range.end {
            let mut ranges = apply_mappings(curr.start..range.start, &mapping_ranges[1..]);
            ranges.extend(apply_mappings(range.start..curr.end, mapping_ranges));
            ranges
        } else {
            let mut ranges = apply_mappings(curr.start..range.end, mapping_ranges);
            ranges.extend(apply_mappings(range.end..curr.end, &mapping_ranges[1..]));
            ranges
        };

        return mapped_ranges;
    };
    
    vec![curr.start..curr.end]
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

fn part_2(input: &str) {
    let (seeds, maps) = input.split_once("\r\n").unwrap();
    let seeds: Vec<_> = seeds[(seeds.find(':').unwrap() + 1)..]
        .split_ascii_whitespace()
        .map(|val| val.parse::<usize>().unwrap())
        .array_chunks()
        .map(|[start, len]| (start..(start + len)))
        .collect();

    let result = maps
        .lines()
        .group_by(|line| line.starts_with(|ch: char| ch.is_ascii_digit()))
        .into_iter()
        .filter(|(matches, _)| *matches)
        .map(|(_, group)| {
            group
                .map(|range| {
                    let (dest, src, len) = range
                        .split_ascii_whitespace()
                        .map(|digit| digit.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    (src..(src + len), dest)
                })
                .collect::<Vec<_>>()
        })
        .fold(seeds, |acc, mapping_ranges| {
            acc.into_iter()
                .flat_map(|mapped_seed| apply_mappings(mapped_seed, &mapping_ranges))
                .collect()
        })
        .into_iter()
        .fold(usize::MAX, |acc, range| cmp::min(acc, range.start));

    println!("Day05 Part 2: {result:?}")
}
