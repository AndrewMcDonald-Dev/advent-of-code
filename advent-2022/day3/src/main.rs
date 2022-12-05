use std::fs;

fn main() {
    let input = fs::read_to_string("in.dat").expect("File not found or could not be read.");
    question_a(&input);
    question_b(&input)
}

fn question_a(input: &str) {
    let result: u32 = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| val(left.chars().find(|c| right.contains(*c)).unwrap()))
        .sum();

    println!("{result}");
}

fn val(c: char) -> u32 {
    let mut ret = u32::from(c);

    ret -= if ret >= 97 { 97 - 1 } else { 65 - 27 };
    ret
}

fn question_b(input: &str) {
    let result: u32 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|sack| match sack[..] {
            [left, mid, right] => val(left
                .chars()
                .find(|c| mid.contains(*c) && right.contains(*c))
                .unwrap()),
            _ => 0,
        })
        .sum();

    println!("{result}")
}
