use std::{cmp, fs};

#[derive(Debug)]
struct Game {
    id: usize,
    max_bag: ColorBag,
}

#[derive(Debug, Default)]
struct ColorBag {
    r: usize,
    g: usize,
    b: usize,
}

impl ColorBag {
    
    fn fit(&self, other: &ColorBag) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }
}

impl From<&str> for ColorBag {
    fn from(round: &str) -> Self {
        let mut bag = ColorBag::default();
        round.split(',').for_each(|color_count| {
            let (count, color) = color_count.trim().split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match color {
                "red" => bag.r = count,
                "green" => bag.g = count,
                "blue" => bag.b = count,
                _ => unreachable!(),
            }
        });

        bag
    }
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (hd, tl) = line.split_once(':').unwrap();
        let id = hd[(hd.find(' ').unwrap() + 1)..].parse().unwrap();

        let max_bag = tl
            .split(';')
            .map(ColorBag::from)
            .reduce(|mut acc, bag| {
                acc.r = cmp::max(acc.r, bag.r);
                acc.g = cmp::max(acc.g, bag.g);
                acc.b = cmp::max(acc.b, bag.b);
                acc
            })
            .unwrap();

        Game { id, max_bag }
    }
}

fn main() {
    let input = fs::read_to_string("./day02/input.txt").unwrap();
    part_1(&input);
}

fn part_1(input: &str) {
    const GIVEN_BAG: ColorBag = ColorBag {
        r: 12,
        g: 13,
        b: 14,
    };

    let result = input
        .split("\r\n")
        .map(Game::from)
        .filter(|game| game.max_bag.fit(&GIVEN_BAG))
        .map(|game| game.id)
        .sum::<usize>();
    println!("Day02 Part 1: {result}")
}
