use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./day01/input.txt").unwrap();

    let (first, second): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(first, second)| {
            (
                first.parse::<usize>().unwrap(),
                second.parse::<usize>().unwrap(),
            )
        })
        .unzip();

    println!("Day01 part 1: {}", part_1(first.clone(), second.clone()));
    println!("Day01 part 2: {}", part_2(first, second));
}

fn part_1(mut first: Vec<usize>, mut second: Vec<usize>) -> usize {
    first.sort();
    second.sort();

    first
        .into_iter()
        .zip(second)
        .map(|(a, b)| usize::abs_diff(a, b))
        .sum::<usize>()
}

fn part_2(first: Vec<usize>, second: Vec<usize>) -> usize {
    let freqs = number_frequencies(second);

    first
        .iter()
        .map(|&num| num * freqs.get(&num).unwrap_or(&0))
        .sum::<usize>()
}

fn number_frequencies(list: Vec<usize>) -> HashMap<usize, usize> {
    let mut freqs = HashMap::new();
    list.iter().for_each(|&num| {
        freqs.entry(num).and_modify(|e| *e += 1).or_insert(1);
    });
    freqs
}
