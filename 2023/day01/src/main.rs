use std::fs;

fn main() {
    let input = fs::read_to_string("./day01/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
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

fn part_2(input: &str) {
    let result = input
        .split("\r\n")
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|idx| {
                let slice = &line[idx..];
                match &slice {
                    _ if slice.starts_with("one") => Some(1),
                    _ if slice.starts_with("two") => Some(2),
                    _ if slice.starts_with("three") => Some(3),
                    _ if slice.starts_with("four") => Some(4),
                    _ if slice.starts_with("five") => Some(5),
                    _ if slice.starts_with("six") => Some(6),
                    _ if slice.starts_with("seven") => Some(7),
                    _ if slice.starts_with("eight") => Some(8),
                    _ if slice.starts_with("nine") => Some(9),
                    _ => slice.chars().next().unwrap().to_digit(10),
                }
            });
            let first_digit = it.next().unwrap();

            first_digit * 10
                + match it.last() {
                    Some(last_digit) => last_digit,
                    None => first_digit,
                }
        })
        .sum::<u32>();
    println!("Day01 Part 2: {result}")
}
