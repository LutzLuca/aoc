use std::fs;

fn main() {
    let input = fs::read_to_string("./day01/input.txt").unwrap();
    part_1(&input);
}

fn part_1(input: &str) {
    let result = input
        .split("\r\n")
        .map(|line| {
            let front_digit = line.chars().find_map(|ch| ch.to_digit(10)).unwrap();
            let back_digit = line
                .chars()
                .rfind(|ch| ch.is_ascii_digit())
                .and_then(|ch| ch.to_digit(10))
                .unwrap();

            front_digit * 10 + back_digit
        })
        .sum::<u32>();
    println!("Day01 Part 1: {result}")
}
