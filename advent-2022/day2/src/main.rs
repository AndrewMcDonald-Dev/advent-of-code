use std::fs;

fn main() {
    let input = fs::read_to_string("in.dat").expect("File not found or could not be read.");
    question_a(&input);
    question_b(&input)
}

fn question_a(input: &str) {
    let score: u32 = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            score_round1(bytes[0], bytes[2]) as u32
        })
        .sum();
    println!("{score}")
}

fn score_round1(opponent: u8, me: u8) -> u8 {
    (me - b'W') + 3 * ((me - opponent + 2) % 3)
}

fn question_b(input: &str) {
    let score: u32 = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            score_round2(bytes[0], bytes[2]) as u32
        })
        .sum();
    println!("{score}")
}

fn score_round2(opponent: u8, outcome: u8) -> u8 {
    let shift = outcome - b'X' + 2; // shift 2 on to lose, 3 (=0 cyclically) to draw, 4=1 to win
    let me = b'X' + (opponent - b'A' + shift) % 3; // calculate shift relative to opponent
    (me - b'W') + 3 * (outcome - b'X')
}
