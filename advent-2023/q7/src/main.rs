use std::{fs, time::Instant};

type Counts = [u32; 13];
struct Hand {
    bid: u32,
    strength: u32,
}

impl Hand {
    fn new(line: &str) -> Self {
        let mut bytes = line.bytes();
        let mut strength = 0u32;

        let mut counts: Counts = [0; 13];
        for i in 0..5 {
            if let Some(card) = bytes.next() {
                let value = match card {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 9,
                    b'T' => 8,
                    n => n - b'0' - 2,
                };
                strength |= (value as u32) << ((4 - i) * 4);
                counts[value as usize] += 1;
            }
        }

        let (max, sec) = find_two_highest(&counts);
        strength |= calculate_type(max, sec) << 20;

        let bid = next_number(&mut bytes).unwrap();

        Hand { bid, strength }
    }

    fn new_with_jokers(line: &str) -> Self {
        let mut bytes = line.bytes();
        let mut strength = 0;
        let mut jokers = 0;

        let mut counts: Counts = [0; 13];
        for i in 0..5 {
            if let Some(card) = bytes.next() {
                let value = match card {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 0,
                    b'T' => 9,
                    n => n - b'0' - 1,
                };

                if value == 0 {
                    jokers += 1;
                } else {
                    counts[value as usize] += 1;
                }

                strength |= (value as u32) << ((4 - i) * 4);
            }
        }

        let (max, sec) = find_two_highest(&counts);
        strength |= calculate_type(max + jokers, sec) << 20;

        let bid = next_number(&mut bytes).unwrap();

        Hand { bid, strength }
    }
}

fn part_1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(Hand::new).collect();

    hands.sort_unstable_by_key(|hand| hand.strength);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize))
}

fn part_2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(Hand::new_with_jokers).collect();

    hands.sort_unstable_by_key(|hand| hand.strength);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize))
}

fn find_two_highest(counts: &Counts) -> (u32, u32) {
    let (mut max, mut sec) = (0, 0);

    for count in counts.iter() {
        if *count > max {
            sec = max;
            max = *count;
        } else if *count > sec {
            sec = *count;
        }
    }

    (max, sec)
}

fn calculate_type(most_matches: u32, second_most_matches: u32) -> u32 {
    match most_matches {
        5 => 6,
        4 => 5,
        3 if second_most_matches == 2 => 4,
        3 => 3,
        2 if second_most_matches == 2 => 2,
        2 => 1,
        _ => 0,
    }
}

fn next_number<T: Iterator<Item = u8>>(input: &mut T) -> Option<u32> {
    let mut value = None;
    for byte in input {
        if let Some(digit) = to_digit(byte) {
            if let Some(current) = value {
                value = Some(current * 10 + digit as u32);
            } else {
                value = Some(digit as u32)
            }
        } else if value.is_some() {
            return value;
        }
    }
    value
}

fn to_digit(byte: u8) -> Option<u8> {
    if byte.is_ascii_digit() {
        return Some(byte - b'0');
    }
    None
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file.");

    println!("--- Day 7: Camel Cards ---");
    let now = Instant::now();
    println!(
        "Part 1: {} | Elapsed: {:.2?}",
        part_1(&input),
        now.elapsed()
    );
    let now = Instant::now();
    println!(
        "Part 2: {} | Elapsed: {:.2?}",
        part_2(&input),
        now.elapsed()
    );
}
