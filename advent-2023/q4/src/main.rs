use std::{fs, time::Instant};

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|card| {
            let mut dupe: u128 = 0;
            let mut score = 1;

            if let Some((_, numbers)) = card.split_once(':') {
                let mut bytes = numbers.bytes();

                while let Some(number) = next_number(&mut bytes) {
                    if 1 << number & dupe > 0 {
                        score *= 2;
                    } else {
                        dupe |= 1 << number;
                    }
                }
            }

            score >> 1
        })
        .sum::<usize>()
}

fn part_2(input: &str) -> u32 {
    let mut card_count: [u32; 250] = [1; 250];
    let mut score = 0;

    for (id, card) in input.lines().enumerate() {
        let count = card_count[id];
        let mut dupe: u128 = 0;
        let mut c = 0;

        if let Some((_, numbers)) = card.split_once(':') {
            let mut bytes = numbers.bytes();

            while let Some(number) = next_number(&mut bytes) {
                if 1 << number & dupe > 0 {
                    c += 1;
                    card_count[id + c] += count;
                } else {
                    dupe |= 1 << number;
                }
            }
        }
        score += count;
    }

    score
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

    println!("--- Day 4: Scratchcards ---");
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
