#![feature(iter_map_windows)]
use std::{cmp::min, fs};

fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i].clone()).collect())
        .collect()
}

fn main() {
    let input = fs::read_to_string("day13/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let valley: Vec<Vec<Vec<_>>> = input
        .split("\r\n\r\n")
        .map(|lines| lines.lines().map(|line| line.chars().collect()).collect())
        .collect();

    let result = valley.into_iter().fold(0, |acc, mirrors| {
        let horz_count = part_1_mirror_axis(&mirrors)
            .map(|pos| pos * 100)
            .unwrap_or_default();

        let vert_count = part_1_mirror_axis(&transpose(mirrors)).unwrap_or_default();

        acc + horz_count + vert_count
    });
    println!("Day13 Part1: {result}")
}

fn part_1_mirror_axis(pattern: &[Vec<char>]) -> Option<usize> {
    pattern
        .iter()
        .enumerate()
        .map_windows(|[(idx, pattern1), (_, pattern2)]| {
            (
                pattern1.iter().zip(pattern2.iter()).all(|(a, b)| a == b),
                *idx,
            )
        })
        .filter_map(|(is_mirror_axis, idx)| is_mirror_axis.then_some(idx))
        .filter(|&idx| {
            let top_offset = idx;
            let bottom_offset = pattern.len() - idx - 2;
            let coord_to_check = std::cmp::min(top_offset, bottom_offset);

            ((idx - coord_to_check)..idx)
                .rev()
                .zip((idx + 2)..(idx + 2 + coord_to_check))
                .all(|(coord1, coord2)| {
                    let p1: &[char] = &pattern[coord1];
                    let p2: &[char] = &pattern[coord2];
                    p1.iter().zip(p2.iter()).all(|(a, b)| a == b)
                })
        })
        .map(|idx| idx + 1)
        .next()
}

fn part_2(input: &str) {
    let valley: Vec<Vec<Vec<_>>> = input
        .split("\r\n\r\n")
        .map(|lines| lines.lines().map(|line| line.chars().collect()).collect())
        .collect();

    let result: usize = valley.into_iter().fold(0, |acc, mirrors| {
        let horz_count = part2_mirror_axis(&mirrors)
            .map(|pos| pos * 100)
            .unwrap_or_default();

        let vert_count = part2_mirror_axis(&transpose(mirrors)).unwrap_or_default();

        acc + horz_count + vert_count
    });
    println!("Day13 Part2: {result}")
}

fn part2_mirror_axis(pattern: &[Vec<char>]) -> Option<usize> {
    pattern
        .iter()
        .enumerate()
        .map_windows(|[(idx, p1), (_, p2)]| (part_2_possible_mirror_axis(p1, p2), *idx))
        .filter_map(|(mirror_axis_opt, idx)| mirror_axis_opt.map(|smudged| (smudged, idx)))
        .find_map(|(mut smudged, idx)| {
            let top_offset = idx;
            let bottom_offset = pattern.len() - idx - 2;
            let coord_to_check = min(top_offset, bottom_offset);

            for (pattern1, pattern2) in ((idx - coord_to_check)..idx)
                .rev()
                .zip((idx + 2)..(idx + 2 + coord_to_check))
                .map(|(coord1, coord2)| (&pattern[coord1], &pattern[coord2]))
            {
                let diff = pattern1
                    .iter()
                    .zip(pattern2.iter())
                    .filter(|(a, b)| a != b)
                    .count();

                if smudged && diff > 0 {
                    return None;
                } else if !smudged && diff == 1 {
                    smudged = true;
                }
            }

            smudged.then_some(idx + 1)
        })
}

fn part_2_possible_mirror_axis(p1: &[char], p2: &[char]) -> Option<bool> {
    let mut is_smudged = false;

    for _ in p1.iter().zip(p2.iter()).filter(|(a, b)| a != b) {
        if is_smudged {
            return None;
        } else {
            is_smudged = true
        }
    }
    Some(is_smudged)
}
