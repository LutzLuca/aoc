use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

struct Hand {
    type_: HandType,
    card_strengths: (u8, u8, u8, u8, u8),
    bid: usize,
}

fn get_hand(cards: &str) -> HandType {
    match cards
        .chars()
        .counts()
        .into_values()
        .sorted()
        .join("")
        .as_str()
    {
        "5" => HandType::FiveOfAKind,
        "14" => HandType::FourOfAKind,
        "23" => HandType::FullHouse,
        "113" => HandType::ThreeOfAKind,
        "122" => HandType::TwoPair,
        "1112" => HandType::OnePair,
        "11111" => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn get_hand_with_joker(cards: &str) -> HandType {
    let freqs = cards.chars().counts();

    let card_counts = if let Some(jokers) = freqs.get(&'J').copied() {
        if jokers == 5 || jokers == 4 {
            return HandType::FiveOfAKind;
        }

        let mut counts: Vec<_> = freqs
            .into_iter()
            .filter_map(|(key, val)| (key != 'J').then_some(val))
            .sorted()
            .collect();

        *counts.last_mut().unwrap() += jokers;

        counts.into_iter().join("")
    } else {
        freqs.into_values().sorted().join("")
    };

    match card_counts.as_str() {
        "5" => HandType::FiveOfAKind,
        "14" => HandType::FourOfAKind,
        "23" => HandType::FullHouse,
        "113" => HandType::ThreeOfAKind,
        "122" => HandType::TwoPair,
        "1112" => HandType::OnePair,
        "11111" => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn main() {
    let input = fs::read_to_string("day07/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let result: usize = input
        .split("\r\n")
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid: usize = bid.parse().unwrap();
            let card_strengths: (u8, u8, u8, u8, u8) = cards
                .chars()
                .map(|card| match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => card.to_digit(10).unwrap() as u8,
                })
                .collect_tuple()
                .unwrap();
            let type_ = get_hand(cards);

            Hand {
                bid,
                card_strengths,
                type_,
            }
        })
        .sorted_by(|hand1, hand2| {
            (hand1.type_ as u8)
                .cmp(&(hand2.type_ as u8))
                .then_with(|| hand1.card_strengths.cmp(&hand2.card_strengths))
        })
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + (idx + 1) * hand.bid);

    println!("Day07 Part 1: {result}")
}

fn part_2(input: &str) {
    let result: usize = input
        .split("\r\n")
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid: usize = bid.parse().unwrap();
            let card_strengths: (u8, u8, u8, u8, u8) = cards
                .chars()
                .map(|card| match card {
                    'A' => 13,
                    'K' => 12,
                    'Q' => 11,
                    'T' => 10,
                    'J' => 1,
                    _ => card.to_digit(10).unwrap() as u8,
                })
                .collect_tuple()
                .unwrap();
            let type_ = get_hand_with_joker(cards);

            Hand {
                bid,
                card_strengths,
                type_,
            }
        })
        .sorted_by(|hand1, hand2| {
            (hand1.type_ as u8)
                .cmp(&(hand2.type_ as u8))
                .then_with(|| hand1.card_strengths.cmp(&hand2.card_strengths))
        })
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + (idx + 1) * hand.bid);

    println!("Day07 Part 2: {result}")
}
