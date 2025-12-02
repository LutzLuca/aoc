use std::fs;

fn main() {
    let input = fs::read_to_string("./day01/input.txt").unwrap();

    println!("Day01 part 1: {}", part_1(input));
}

fn part_1(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let (rotation, distance) = line.split_at(1);
            let distance = distance.parse::<isize>().unwrap();

            distance
                * match rotation {
                    "L" => -1,
                    "R" => 1,
                    _ => unreachable!(),
                }
        })
        .fold((50, 0), |(dial, zeros), rotation| {
            let dial = (dial + rotation) % 100;
            let zeros = zeros + (dial == 0) as usize;
            (dial, zeros)
        })
        .1
}
