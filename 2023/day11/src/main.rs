use std::{cmp, fs};

fn main() {
    let input = fs::read_to_string("day11/input.txt").unwrap();
    part_1(&input);
}

fn part_1(input: &str) {
    let image: Vec<Vec<_>> = input.lines().map(|row| row.chars().collect()).collect();
    let galaxies: Vec<_> = image
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &ch)| ch == '#')
                .map(move |(col_idx, _)| (row_idx, col_idx))
        })
        .collect();

    let empty_cols: Vec<_> = (0..image[0].len())
        .filter(|&col_idx| image.iter().all(|row| row[col_idx] == '.'))
        .collect();

    let empty_rows: Vec<_> = image
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| row.iter().all(|&ch| ch == '.').then_some(row_idx))
        .collect();

    let dist = |curr, pos| dist(curr, pos, &empty_rows, &empty_cols);

    let result = galaxies
        .iter()
        .enumerate()
        .map(|(idx, curr)| {
            galaxies[(idx + 1)..]
                .iter()
                .fold(0, |acc, &galaxy| acc + dist(*curr, galaxy))
        })
        .sum::<usize>();
    println!("Day11 Part1: {result}")
}

fn dist(
    (curr_row, curr_col): (usize, usize),
    (goal_row, goal_col): (usize, usize),
    empty_rows: &[usize],
    empty_cols: &[usize],
) -> usize {
    let (min_row, max_row) = (cmp::min(curr_row, goal_row), cmp::max(curr_row, goal_row));
    let (min_col, max_col) = (cmp::min(curr_col, goal_col), cmp::max(curr_col, goal_col));

    let row_range = (min_row + 1)..max_row;
    let col_range = (min_col + 1)..max_col;

    let extended_rows = empty_rows
        .iter()
        .filter(|row| row_range.contains(row))
        .count();

    let extended_cols = empty_cols
        .iter()
        .filter(|col| col_range.contains(col))
        .count();

    let rows = max_row - min_row - extended_rows + 2 * extended_rows;
    let cols = max_col - min_col - extended_cols + 2 * extended_cols;

    rows + cols
}
