use itertools::Itertools;
use std::fs;

fn part_1_direction_count(line: &str) -> ((isize, isize), isize) {
    let (dir, count, _) = line.split(' ').collect_tuple().unwrap();
    let dir = {
        match dir.chars().next() {
            Some('R') => (1, 0),
            Some('L') => (-1, 0),
            Some('U') => (0, -1),
            Some('D') => (0, 1),
            _ => unreachable!(),
        }
    };
    let count = count.parse::<isize>().unwrap();
    (dir, count)
}

fn part_2_direction_count(line: &str) -> ((isize, isize), isize) {
    let color = line.split(' ').last().unwrap();
    let color = &color[2..color.len() - 1];
    let dir = {
        match color.chars().last() {
            Some('0') => (1, 0),
            Some('2') => (-1, 0),
            Some('3') => (0, -1),
            Some('1') => (0, 1),
            _ => unreachable!(),
        }
    };
    let count = isize::from_str_radix(&color[..5], 16).unwrap();
    (dir, count)
}

fn compute_lava_holdings(
    boundary_points: &[(isize, isize)],
    boundary_points_count: usize,
) -> usize {
    // Shoelace formula for area of a Polygon
    let inner_area = boundary_points
        .iter()
        .zip(boundary_points.iter().cycle().skip(1))
        .fold(0, |acc, ((x1, y1), (x2, y2))| acc + (x1 * y2) - (x2 * y1))
        .abs() as f64
        / 2.0;

    // Result = Boundary_Lattice_Points + Interior_Lattice_Points
    // Pick's theorem for computing Interior_Lattice_Points
    boundary_points_count + (inner_area - boundary_points_count as f64 / 2.0 + 1.0) as usize
}

fn main() {
    let input = fs::read_to_string("day18/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let (boundary_points, boundary_points_count): (Vec<_>, usize) =
        input.split("\r\n").map(part_1_direction_count).fold(
            // Input forms a loop around (0,0)
            // so no need for it to be in the init
            // as this wound include it twice
            (vec![], 0),
            |(mut acc, total), ((dx, dy), count)| {
                let next_point = acc.last().map_or_else(
                    || (dx * count, dy * count),
                    |(x, y)| (x + dx * count, y + dy * count),
                );
                acc.push(next_point);
                (acc, total + count as usize)
            },
        );

    println!(
        "Day18 Part1: {}",
        compute_lava_holdings(&boundary_points, boundary_points_count)
    )
}

fn part_2(input: &str) {
    let (boundary_points, boundary_points_count): (Vec<_>, usize) = input
        .split("\r\n")
        .map(part_2_direction_count)
        .fold((vec![], 0), |(mut acc, total), ((dx, dy), count)| {
            let next_point = acc.last().map_or_else(
                || (dx * count, dy * count),
                |(x, y)| (x + dx * count, y + dy * count),
            );
            acc.push(next_point);
            (acc, total + count as usize)
        });

    println!(
        "Day18 Part2: {}",
        compute_lava_holdings(&boundary_points, boundary_points_count)
    )
}
