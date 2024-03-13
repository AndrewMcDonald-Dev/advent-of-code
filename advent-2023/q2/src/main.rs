use std::{fs, time::Instant};

fn part_1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut bytes = line.bytes();
            let _ = next_number(&mut bytes);

            while let Some(value) = next_number(&mut bytes) {
                // let value = value;
                match bytes.next().unwrap() {
                    b'r' if value > 12 => return 0,
                    b'g' if value > 13 => return 0,
                    b'b' if value > 14 => return 0,
                    _ => {}
                }
            }
            id + 1
        })
        .sum::<usize>()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            let mut bytes = line.bytes();
            let _ = next_number(&mut bytes);

            while let Some(value) = next_number(&mut bytes) {
                match bytes.next().unwrap() {
                    b'r' if red < value => red = value,
                    b'g' if green < value => green = value,
                    b'b' if blue < value => blue = value,
                    _ => {}
                }
            }

            red * green * blue
        })
        .sum::<u32>()
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

    println!("--- Day 2: Cube Conundrum ---");
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
