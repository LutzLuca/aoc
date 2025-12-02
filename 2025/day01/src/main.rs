use std::fs;

fn main() {
    let input = fs::read_to_string("./day01/input.txt").unwrap();

    println!("Day01 part 1: {}", part_1(&input));
    println!("Day01 part 2: {}", part_2(&input));
}

fn part_2(input: &str) -> usize {
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
        .fold((50, 0), |(mut dial, mut zeros), mut rotation| {
            let dir = rotation.signum();

            loop {
                let dist = distance_to_zero(dial, dir);

                if rotation.is_negative() {
                    if dist < rotation {
                        break;
                    }
                } else if dist > rotation {
                    break;
                }

                rotation -= dist;
                zeros += 1;
                dial = 0;

                if rotation == 0 {
                    break;
                }
            }

            let dial = dial
                .checked_add_signed(rotation)
                .unwrap_or((rotation + 100) as usize);

            (dial, zeros)
        })
        .1
}

fn distance_to_zero(dial: usize, dir_signum: isize) -> isize {
    assert!(dir_signum != 0);
    if dial == 0 {
        return dir_signum * 100;
    }

    let dial = dial as isize;
    match dir_signum {
        1 => 100 - dial,
        -1 => -dial,
        _ => unreachable!(),
    }
}

fn part_1(input: &str) -> usize {
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
