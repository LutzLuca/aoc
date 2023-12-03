use std::{fs, ops::RangeInclusive};

#[derive(Debug)]
struct Number {
    row: usize,
    span: RangeInclusive<usize>,
    val: usize,
    found: bool,
}

const fn is_symbol(ch: char) -> bool {
    !ch.is_ascii_digit() && ch != '.'
}

fn main() {
    let input = fs::read_to_string("./day03/input.txt").unwrap();
    part_1(&input);
    // part_1_first_sol(&input)
}

fn part_1(input: &str) {
    let mut nums: Vec<Number> = vec![];
    let mut syms: Vec<(usize, usize)> = vec![];

    for (r_idx, row) in input.split("\r\n").enumerate() {
        let mut chs = row.char_indices().peekable();
        while let Some((c_idx, ch)) = chs.next() {
            if is_symbol(ch) {
                syms.push((r_idx, c_idx));
                continue;
            } else if ch.is_ascii_digit() {
                let start = c_idx;
                let mut end = c_idx;

                while chs.peek().is_some_and(|(_, val)| val.is_ascii_digit()) {
                    chs.next();
                    end += 1;
                }

                nums.push(Number {
                    row: r_idx,
                    span: start..=end,
                    val: row[start..=end].parse().unwrap(),
                    found: false,
                });
            }
        }
    }

    let mut result = 0;
    syms.into_iter().for_each(|(row, col)| {
        for dx in -1..=1 {
            for dy in -1..=1 {
                let row = std::cmp::max(0, row as i32 + dy) as usize;
                let col = std::cmp::max(0, col as i32 + dx) as usize;

                nums.iter_mut()
                    .filter(|num| num.row == row && !num.found && num.span.contains(&col))
                    .for_each(|num| {
                        num.found = true;
                        result += num.val;
                    })
            }
        }
    });

    println!("Day03 Part 1: {result}")
}

fn part_1_first_sol(input: &str) {
    let mut ranges: Vec<(usize, RangeInclusive<usize>)> = Vec::new();

    let board: Vec<Vec<_>> = input
        .split("\r\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let rows = board.len();
    let cols = board[0].len();

    board
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_digit() && **c != '.')
                .map(move |(col, _)| (row_idx, col))
        })
        .for_each(|(row, col)| {
            for dx in -1..1 {
                for dy in -1..1 {
                    let row = std::cmp::max(0, row as i32 + dy) as usize;
                    let col = std::cmp::max(0, col as i32 + dx) as usize;

                    if row < rows && col < cols {
                        if ranges
                            .iter()
                            .filter(|(r, _)| *r == row)
                            .any(|(_, range)| range.contains(&col))
                        {
                            continue;
                        }

                        let mut start = col;
                        let mut end = col;

                        if board[row][col].is_ascii_digit() {
                            while start > 0 && board[row][start - 1].is_ascii_digit() {
                                start -= 1;
                            }
                            while end < (board[0].len() - 1) && board[row][end + 1].is_ascii_digit()
                            {
                                end += 1;
                            }

                            ranges.push((row, start..=end))
                        }
                    }
                }
            }
        });

    let result = ranges
        .into_iter()
        .map(|(row, range)| {
            String::from_iter(&board[row][range])
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>();
    println!("Day03 Part1: {result}")
}
