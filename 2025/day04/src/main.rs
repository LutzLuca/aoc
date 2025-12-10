use std::fs;

fn main() {
    let input = fs::read_to_string("./day04/input.txt").unwrap();

    let grid = input
        .lines()
        .map(|line| line.chars().map(|ch| ch == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Day04 part 1: {}", part_1(&grid));
    println!("Day04 part 2: {}", part_2(grid));
}

fn part_2(mut grid: Vec<Vec<bool>>) -> usize {
    let mut count = 0;

    let mut to_remove: Vec<(usize, usize)> = Vec::new();
    let mut initial = true;
    while !to_remove.is_empty() || initial {
        to_remove.drain(..).for_each(|(r, c)| grid[r][c] = false);
        initial = false;

        for (r, row) in grid.iter().enumerate() {
            for (c, is_paper_roll) in row.iter().enumerate() {
                if !is_paper_roll {
                    continue;
                }

                if count_adjacent_paper_rolls((r, c), &grid) < 4 {
                    to_remove.push((r, c));
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_1(grid: &[Vec<bool>]) -> usize {
    let mut count = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, is_paper_roll) in row.iter().enumerate() {
            if !is_paper_roll {
                continue;
            }

            if count_adjacent_paper_rolls((r, c), grid) < 4 {
                count += 1;
            }
        }
    }

    count
}

fn count_adjacent_paper_rolls(pos: (usize, usize), grid: &[Vec<bool>]) -> usize {
    adjacent_positions(pos, grid)
        .filter(|&(x, y)| grid[x][y])
        .count()
}

fn adjacent_positions(
    (curr_x, curr_y): (usize, usize),
    grid: &[Vec<bool>],
) -> impl Iterator<Item = (usize, usize)> {
    const OFFSETS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (0, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
        (1, 0),
        (0, 1),
    ];

    let (rows, cols) = (grid[0].len(), grid.len());

    OFFSETS.iter().filter_map(move |&(offset_x, offset_y)| {
        let adjacent_x = curr_x
            .checked_add_signed(offset_x)
            .and_then(|adjacent_x| (adjacent_x < rows).then_some(adjacent_x));
        let adjacent_y = curr_y
            .checked_add_signed(offset_y)
            .and_then(|adjacent_y| (adjacent_y < cols).then_some(adjacent_y));

        Option::zip(adjacent_x, adjacent_y)
    })
}
