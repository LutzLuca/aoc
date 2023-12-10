use std::{collections::BTreeMap, fs};

const fn lcm(a: usize, b: usize) -> usize {
    const fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    a * b / gcd(a, b)
}

fn main() {
    let input = fs::read_to_string("day08/input.txt").unwrap();

    part_1(&input);
    #[cfg(not(debug_assertions))]
    part_1_rec(&input);

    part_2(&input);
}

fn part_2(input: &str) {
    let (instructions, network) = input.split_once("\r\n\r\n").unwrap();

    let network: BTreeMap<_, _> = network
        .lines()
        .map(|line| {
            let (curr, next) = line.split_once(" = ").unwrap();
            let (left, right) = next[1..(next.len() - 1)].split_once(", ").unwrap();

            (curr, (left, right))
        })
        .collect();

    // input is constructed in such a way that the cycle len = idx of first occurrence that ends with 'Z',
    // that is included in that cycle. Therefore landing on that occurrence takes (n * cycle len) steps.
    // => finding the lcm of all the cycle lengths is the answer
    let result = network
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .map(|mut curr| {
            instructions
                .chars()
                .cycle()
                .enumerate()
                .find_map(|(step, dir)| {
                    curr.ends_with('Z').then_some(step).or({
                        curr = match dir {
                            'L' => network.get(curr).unwrap().0,
                            'R' => network.get(curr).unwrap().1,
                            _ => unreachable!(),
                        };
                        None
                    })
                })
                .unwrap()
        })
        .reduce(lcm)
        .unwrap();
    println!("Day08 Part 2: {result}")
}

fn part_1(input: &str) {
    let (instructions, network) = input.split_once("\r\n\r\n").unwrap();

    let network: BTreeMap<_, _> = network
        .lines()
        .map(|line| {
            let (curr, next) = line.split_once(" = ").unwrap();
            let (left, right) = next[1..(next.len() - 1)].split_once(", ").unwrap();

            (curr, (left, right))
        })
        .collect();
    let mut curr = "AAA";

    let result = instructions
        .chars()
        .cycle()
        .enumerate()
        .find_map(|(step, dir)| match curr {
            "ZZZ" => Some(step),
            _ => {
                curr = match dir {
                    'L' => network.get(curr).unwrap().0,
                    'R' => network.get(curr).unwrap().1,
                    _ => unreachable!(),
                };
                None
            }
        })
        .unwrap();

    println!("Day08 Part 1: {result}");
}

#[cfg(not(debug_assertions))]
// Only worked for me in release build
fn part_1_rec(input: &str) {
    fn count_steps_to_end(
        curr: &str,
        network: &BTreeMap<&str, (&str, &str)>,
        instructions: &str,
        steps_taken: usize,
    ) -> usize {
        match curr {
            "ZZZ" => steps_taken,
            _ => {
                let dir = instructions.as_bytes()[steps_taken % instructions.len()];
                let next = match dir {
                    b'L' => network.get(curr).unwrap().0,
                    b'R' => network.get(curr).unwrap().1,
                    _ => unreachable!(),
                };
                count_steps_to_end(next, network, instructions, steps_taken + 1)
            }
        }
    }

    let (instructions, network) = input.split_once("\r\n\r\n").unwrap();

    let network: BTreeMap<_, _> = network
        .lines()
        .map(|line| {
            let (curr, next) = line.split_once(" = ").unwrap();
            let (left, right) = next[1..(next.len() - 1)].split_once(", ").unwrap();

            (curr, (left, right))
        })
        .collect();
    let start = "AAA";

    let result = count_steps_to_end(start, &network, instructions, 0);

    println!("Day08 Part 1: {result}");
}
