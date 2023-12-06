use std::fs;

fn main() {
    let input = fs::read_to_string("day06/input.txt").unwrap();
    part_1(&input);
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
