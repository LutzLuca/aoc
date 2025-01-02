use regex::Regex;
use std::fs;

#[derive(Clone, Copy)]
enum Status {
    Enabeled,
    Disabeled,
}

enum Instructions {
    Mul { x: usize, y: usize },
    Enabeled,
    Disabeled,
}

fn main() {
    let input = fs::read_to_string("./day03/input.txt").unwrap();
    let r = Regex::new(r#"mul\(([0-9]+),([0-9]+)\)"#).unwrap();

    let res: usize = r
        .captures_iter(&input)
        .map(|capt| capt.extract())
        .map(|(_, [x, y])| x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap())
        .sum();

    let r2 = Regex::new(r#"mul\(([0-9]+),([0-9]+)\)|(do\(\)|don't\(\))()"#).unwrap();

    let (res2, _) = r2
        .captures_iter(&input)
        .map(|cap| match cap.get(0).map(|m| m.as_str()) {
            Some("do()") => Instructions::Enabeled,
            Some("don't()") => Instructions::Disabeled,
            None => unreachable!(),
            _ => {
                let (_, [x, y]) = cap.extract();
                Instructions::Mul {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            }
        })
        .fold((0, Status::Enabeled), |(acc, status), inst| {
            match (inst, status) {
                (Instructions::Mul { x, y }, Status::Enabeled) => (acc + x * y, status),
                (Instructions::Enabeled, Status::Disabeled) => (acc, Status::Enabeled),
                (Instructions::Disabeled, Status::Enabeled) => (acc, Status::Disabeled),
                _ => (acc, status),
            }
        });

    println!("Day03 Part 1: {res}");
    println!("Day03 Part 2: {res2}");
}
