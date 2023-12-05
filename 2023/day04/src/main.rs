use std::fs;

fn main() {
    let input = fs::read_to_string("./day04/input.txt").unwrap();
    part_1(&input);
    part_2(&input)
}

fn part_1(input: &str) {
    let result = input
        .split("\r\n")
        .map(|line| {
            let (_, scratchcard) = line.split_once(": ").unwrap();
            let (winner_cards, draws) = scratchcard.split_once(" | ").unwrap();
            let winner_cards: Vec<_> = winner_cards.split_ascii_whitespace().collect();

            draws
                .split_ascii_whitespace()
                .filter(|card| winner_cards.contains(card))
                .count()
        })
        .filter(|&count| count > 0)
        .fold(0, |acc, count| acc + 2usize.pow(count as u32 - 1));

    println!("Day04 Part 1: {result}")
}

fn part_2(input: &str) {
    let scores: Vec<_> = input
        .split("\r\n")
        .map(|line| {
            let (_, scratchcard) = line.split_once(": ").unwrap();
            let (winners, draws) = scratchcard.split_once(" | ").unwrap();
            let winners: Vec<_> = winners.split_ascii_whitespace().collect();

            draws
                .split_ascii_whitespace()
                .filter(|card| winners.contains(card))
                .count()
        })
        .collect();
    let freqs = vec![1; scores.len()];

    let result: usize = scores
        .iter()
        .enumerate()
        .fold(freqs, |mut acc, (idx, score)| {
            let copies = acc[idx];

            acc[(idx + 1)..(idx + 1 + score)]
                .iter_mut()
                .for_each(|val| *val += copies);

            acc
        })
        .into_iter()
        .sum();

    println!("Day04 Part 2: {result}")
}
