use std::fs;

fn hash(seq: &str) -> usize {
    seq.bytes()
        .fold(0, |acc, ch| (acc + ch as usize) * 17 % 256)
}

fn main() {
    let input = fs::read_to_string("day15/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_2(input: &str) {
    let result: usize = input
        .split(',')
        .fold(vec![vec![]; 256], |mut boxes, seq| {
            let (seq, op) = seq.split_at(seq.chars().position(|ch| !ch.is_alphabetic()).unwrap());
            let box_idx = hash(seq);

            match op.as_bytes() {
                [b'=', len] => {
                    const ASCII_DIGIT_OFFSET: u8 = 48;
                    let focal_len = (len - ASCII_DIGIT_OFFSET) as usize;

                    let curr_box = &mut boxes[box_idx];
                    if let Some(idx) = curr_box.iter().position(|(s, _)| *s == seq) {
                        curr_box[idx].1 = focal_len;
                    } else {
                        curr_box.push((seq, focal_len));
                    }
                }
                [b'-'] => {
                    if let Some(idx) = boxes[box_idx].iter().position(|&(s, _)| s == seq) {
                        boxes[box_idx].remove(idx);
                    }
                }
                _ => unreachable!(),
            };

            boxes
        })
        .into_iter()
        .enumerate()
        .map(|(box_idx, curr_box)| {
            curr_box
                .into_iter()
                .enumerate()
                .map(|(lens_idx, (_, focal_len))| (box_idx + 1) * (lens_idx + 1) * focal_len)
                .sum::<usize>()
        })
        .sum();

    println!("Day15 part2: {result}");
}

fn part_1(input: &str) {
    let result: usize = input.split(',').map(hash).sum();

    println!("Day15 part1: {result}");
}
