use std::{fs, time::Instant};

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn solutions(&self) -> u64 {
        let distance = (self.time * self.time - 4 * (self.distance + 1)) as f64;
        let distance = distance.sqrt();
        let mut low = (self.time as f64 - distance) / 2.0;
        let mut high = (self.time as f64 + distance) / 2.0;

        if low.fract() > f64::EPSILON {
            low = low.ceil();
        }

        if high.fract() < f64::EPSILON {
            high = high.floor();
        }

        (high - low) as u64 + 1
    }
}

fn parse_races<A, B>(times: &mut A, distances: &mut B) -> Vec<Race>
where
    A: Iterator<Item = u8>,
    B: Iterator<Item = u8>,
{
    let mut races = Vec::new();

    while let Some(time) = next_number(times) {
        if let Some(distance) = next_number(distances) {
            races.push(Race { time, distance });
        }
    }
    races
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut times = lines.next().unwrap().bytes();
    let mut distances = lines.next().unwrap().bytes();
    let races = parse_races(&mut times, &mut distances);

    races
        .iter()
        .map(|race| race.solutions())
        .fold(1, |acc, val| acc * val as usize)
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut times = lines
        .next()
        .unwrap()
        .bytes()
        .filter(|char| char.is_ascii_digit());

    let mut distances = lines
        .next()
        .unwrap()
        .bytes()
        .filter(|char| char.is_ascii_digit());

    let races = parse_races(&mut times, &mut distances);

    races
        .iter()
        .map(|race| race.solutions())
        .fold(1, |acc, val| acc * val as usize)
}

fn next_number<T: Iterator<Item = u8>>(input: &mut T) -> Option<u64> {
    let mut value = None;
    for byte in input {
        if let Some(digit) = to_digit(byte) {
            if let Some(current) = value {
                value = Some(current * 10 + digit as u64);
            } else {
                value = Some(digit as u64)
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

    println!("--- Day 6: Wait For It ---");
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
