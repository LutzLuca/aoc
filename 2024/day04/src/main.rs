#![feature(iter_map_windows)]
use std::fs;

fn main() {
    let input = fs::read_to_string("./day04/input.txt").unwrap();
    let word_search: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (word_search.len(), word_search[0].len());

    let part_1_res = count_total_xmas(&word_search);
    let part_2_res: usize = (0..rows)
        .flat_map(move |row| (0..cols).map(move |col| (col, row)))
        .filter(|&(x, y)| word_search[y][x] == 'A')
        .filter_map(|coord| is_x_mas(coord, &word_search))
        .sum();

    println!("Day04 Part 1: {part_1_res}");
    println!("Day04 Part 2: {part_2_res}");
}

fn is_x_mas((a_x, a_y): (usize, usize), word_search: &[Vec<char>]) -> Option<usize> {
    fn out_of_bounds((x, y): (usize, usize), (rows, cols): (usize, usize)) -> bool {
        x >= cols || y >= rows
    }
    fn pattern_is_valid(s: String) -> bool {
        s == "SM" || s == "MS"
    }
    let dims = (word_search.len(), word_search[0].len());

    assert!(word_search[a_y][a_x] == 'A');
    const DIRS: [[(isize, isize); 2]; 2] = [[(1, 1), (-1, -1)], [(-1, 1), (1, -1)]];

    DIRS.iter()
        .map(|dir| {
            String::from_iter(
                dir.iter()
                    .filter_map(|&(dx, dy)| {
                        Option::zip(a_x.checked_add_signed(dx), a_y.checked_add_signed(dy))
                            .and_then(|point| (!out_of_bounds(point, dims)).then_some(point))
                    })
                    .map(|(x, y)| word_search[y][x]),
            )
        })
        .all(pattern_is_valid)
        .then_some(1)
}

fn count_total_xmas(word_search: &[Vec<char>]) -> usize {
    fn get_xmax_count(s: Vec<String>) -> usize {
        s.into_iter().map(count_xmas).sum()
    }

    [
        get_cols(word_search),
        get_rows(word_search),
        get_diags(word_search),
        get_anit_diags(word_search),
    ]
    .into_iter()
    .map(get_xmax_count)
    .sum()
}

fn count_xmas(word: String) -> usize {
    fn is_xmas(chars: [char; 4]) -> usize {
        let str = String::from_iter(chars);

        (str == "XMAS").then_some(1).unwrap_or_default()
            + (String::from_iter(chars.iter().rev()) == "XMAS")
                .then_some(1)
                .unwrap_or_default()
    }

    word.chars()
        .map_windows(|sub_str: &[char; 4]| is_xmas(*sub_str))
        .sum()
}

fn get_cols(word_search: &[Vec<char>]) -> Vec<String> {
    word_line_from_iter(
        (0..word_search[0].len()).map(|col| (col, 0)),
        (0, 1),
        word_search,
    )
}

fn get_rows(word_search: &[Vec<char>]) -> Vec<String> {
    word_line_from_iter(
        (0..word_search.len()).map(|row| (0, row)),
        (1, 0),
        word_search,
    )
}

fn get_diags(word_search: &[Vec<char>]) -> Vec<String> {
    let (rows, cols) = (word_search.len(), word_search[0].len());
    word_line_from_iter(
        (0..rows)
            .map(|row| (0, row))
            .chain((1..cols).map(|col| (col, 0))),
        (1, 1),
        word_search,
    )
}

fn get_anit_diags(word_search: &[Vec<char>]) -> Vec<String> {
    let (rows, cols) = (word_search.len(), word_search[0].len());

    word_line_from_iter(
        (0..rows)
            .map(|row| (0, row))
            .chain((1..cols).map(|col| (col, rows - 1))),
        (1, -1),
        word_search,
    )
}

fn word_line_from_iter<I>(iter: I, dir: (isize, isize), word_search: &[Vec<char>]) -> Vec<String>
where
    I: Iterator<Item = (usize, usize)>,
{
    iter.flat_map(|start| line_from_point(start, dir, (word_search.len(), word_search[0].len())))
        .map(|point| String::from_iter(point.into_iter().map(|(x, y)| word_search[y][x])))
        .collect()
}

fn line_from_point(
    start: (usize, usize),
    (dx, dy): (isize, isize),
    (rows, cols): (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut points = vec![start];

    let (mut x, mut y) = start;
    while let Some(point) = Option::zip(x.checked_add_signed(dx), y.checked_add_signed(dy))
        .and_then(|(x, y)| (x < cols && y < rows).then_some((x, y)))
    {
        points.push(point);

        (x, y) = point;
    }

    (points.len() >= 4).then_some(points)
}
