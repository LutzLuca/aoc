use std::{fs, iter, usize};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    const fn to_offset(self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

impl Pipe {
    const fn connections(&self) -> (Direction, Direction) {
        match self {
            Pipe::Vertical => (Direction::North, Direction::South),
            Pipe::Horizontal => (Direction::West, Direction::East),
            Pipe::NorthWest => (Direction::North, Direction::West),
            Pipe::NorthEast => (Direction::North, Direction::East),
            Pipe::SouthWest => (Direction::South, Direction::West),
            Pipe::SouthEast => (Direction::South, Direction::East),
            _ => unimplemented!(),
        }
    }
}

impl From<char> for Pipe {
    fn from(ch: char) -> Self {
        match ch {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => unimplemented!(),
        }
    }
}

fn get_loop_coords(maze: &[Vec<Pipe>]) -> impl Iterator<Item = (usize, usize)> + '_ {
    let (sr, sc) = maze
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .position(|pipe| matches!(pipe, Pipe::Start))
                .map(|c| (r, c))
        })
        .unwrap();

    // I looked at the input and change the start postion and direction
    // matching the next pipe from the Start in the loop
    iter::successors(
        Some(((sr, sc + 1), Direction::East)),
        move |&((row, col), last_out_dir)| {
            let curr = maze[row][col];
            if matches!(curr, Pipe::Start) {
                return None;
            }

            let in_dir = last_out_dir.opposite();
            let out_dir = match curr.connections() {
                (dir1, dir2) if dir1 == in_dir => dir2,
                (dir1, dir2) if dir2 == in_dir => dir1,
                _ => unreachable!(),
            };
            let (drow, dcol) = out_dir.to_offset();

            let next_coord = (
                (row as isize + drow) as usize,
                (col as isize + dcol) as usize,
            );

            Some((next_coord, out_dir))
        },
    )
    .map(|(pos, _)| pos)
}

fn main() {
    let input = fs::read_to_string("day10/input.txt").unwrap();
    let maze: Vec<Vec<_>> = input
        .split("\r\n")
        .map(|row| row.chars().map(Pipe::from).collect())
        .collect();

    // part_1_rec(&maze);
    part_1(&maze);
    part_2(&maze);
}

fn part_2(maze: &[Vec<Pipe>]) {
    let boundary_points: Vec<_> = get_loop_coords(maze).collect();

    // Shoelace formula for the area of a polygon
    let area = boundary_points
        .iter()
        .zip(boundary_points.iter().cycle().skip(1))
        .fold(0, |acc, ((x1, y1), (x2, y2))| {
            acc + (x1 * y2) as isize - (x2 * y1) as isize
        })
        .abs() as f64
        / 2.0;

    // Pick's theorem for a polygon:
    // Area = Interior_Lattice_Points + Boundary_Lattice_Points / 2 - 1
    // => Interior_Lattice_Points = Area - Boundary_Lattice_Points / 2 + 1
    let interior_lattice_points = area - (boundary_points.len() / 2) as f64 + 1.0;

    println!("Day10 Part 2: {interior_lattice_points}");
}

fn part_1(maze: &[Vec<Pipe>]) {
    println!("Day10 Part 1: {}", get_loop_coords(maze).count() / 2)
}

#[allow(dead_code)]
fn part_1_rec(maze: &[Vec<Pipe>]) {
    fn traverse_loop(
        (row, col): (usize, usize),
        maze: &[Vec<Pipe>],
        in_dir: Direction,
        step: usize,
    ) -> usize {
        let curr = maze[row][col];
        if matches!(curr, Pipe::Start) {
            return step;
        }

        let out_dir = match curr.connections() {
            (dir1, dir2) if dir1 == in_dir => dir2,
            (dir1, dir2) if dir2 == in_dir => dir1,
            _ => unreachable!(),
        };
        let (drow, dcol) = out_dir.to_offset();

        let next_coord = (
            (row as isize + drow) as usize,
            (col as isize + dcol) as usize,
        );

        traverse_loop(next_coord, maze, out_dir.opposite(), step + 1)
    }

    let (start_row, start_col) = maze
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .position(|pipe| matches!(pipe, Pipe::Start))
                .map(|col_idx| (r, col_idx))
        })
        .unwrap();

    println!(
        "Day10 Part 1: {}",
        traverse_loop((start_row, start_col + 1), maze, Direction::West, 1) / 2
    );
}
