use std::{collections::HashMap, fs};

fn move_east(grid: &mut [Vec<char>]) {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for col in (0..grid[0].len()).rev() {
        for row in 0..grid.len() {
            if grid[row][col] == '#' {
                map.insert(row, col);
                continue;
            }
            if grid[row][col] == 'O' {
                let x = *map
                    .entry(row)
                    .and_modify(|c| *c -= 1)
                    .or_insert(grid[0].len() - 1);
                grid[row][col] = '.';
                grid[row][x] = 'O';
            }
        }
    }
}

fn move_west(grid: &mut [Vec<char>]) {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for col in 0..grid[0].len() {
        for (r, row) in grid.iter_mut().enumerate() {
            if row[col] == '#' {
                map.insert(r, col);
                continue;
            }
            if row[col] == 'O' {
                let x = *map.entry(r).and_modify(|c| *c += 1).or_insert(0);
                row[col] = '.';
                row[x] = 'O';
            }
        }
    }
}

fn move_north(grid: &mut [Vec<char>]) {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                map.insert(col, row);
                continue;
            }
            if grid[row][col] == 'O' {
                let y = *map.entry(col).and_modify(|r| *r += 1).or_insert(0);
                grid[row][col] = '.';
                grid[y][col] = 'O';
            }
        }
    }
}

fn move_south(grid: &mut [Vec<char>]) {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for row in (0..grid.len()).rev() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                map.insert(col, row);
                continue;
            }
            if grid[row][col] == 'O' {
                let y = *map
                    .entry(col)
                    .and_modify(|r| *r -= 1)
                    .or_insert(grid.len() - 1);
                grid[row][col] = '.';
                grid[y][col] = 'O';
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("day14/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_2(input: &str) {
    let mut grid: Vec<Vec<_>> = input
        .split("\r\n")
        .map(|line| line.chars().collect())
        .collect();
    let max_height = grid.len();

    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut north_loads: Vec<_> = vec![];

    let loop_begin = [move_north, move_west, move_south, move_east]
        .chunks(4)
        .cycle()
        .find_map(|period| {
            period
                .iter()
                .for_each(|movement_func| movement_func(&mut grid));

            let grid_str = grid.iter().fold(String::new(), |mut acc, row| {
                acc.extend(row);
                acc
            });

            match cache.get(&grid_str) {
                Some(idx) => Some(*idx),
                None => {
                    let north_load = grid.iter().enumerate().fold(0, |acc, (r, row)| {
                        acc + (max_height - r) * row.iter().filter(|ch| **ch == 'O').count()
                    });
                    north_loads.push(north_load);
                    cache.insert(grid_str, cache.len());

                    None
                }
            }
        })
        .unwrap();

    let left_over_cycles = (1_000_000_000 - loop_begin) % (north_loads.len() - loop_begin);
    let result = north_loads[left_over_cycles + loop_begin - 1];
    println!("Day14 Part2: {result}");
}

fn part_1(input: &str) {
    fn move_north((x, y): (usize, usize), grid: &mut [Vec<char>]) -> usize {
        if y > 0 && grid[y - 1][x] == '.' {
            grid[y][x] = '.';
            grid[y - 1][x] = 'O';
            return move_north((x, y - 1), grid);
        }
        y
    }

    let mut grid: Vec<Vec<_>> = input
        .split("\r\n")
        .map(|line| line.chars().collect())
        .collect();
    let grid_height = grid.len();

    let rolling_stones: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, ch)| **ch == 'O')
                .map(move |(c, _)| (c, r))
        })
        .collect();

    let result = rolling_stones
        .into_iter()
        .fold(0, |acc, pos| acc + grid_height - move_north(pos, &mut grid));
    println!("Day14 Part1: {result}");
}
