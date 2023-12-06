use std::fs;

fn main() {
    let input = fs::read_to_string("in.dat").expect("File not found.");
    let question_a = input.lines().map(question_a).sum::<u32>();
    println!("PART 1: {}", question_a);
    // question_b(&input);
    let question_b = input.lines().map(question_b).sum::<u32>();
    println!("PART 2: {}", question_b);
}

fn question_a(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
}

fn question_b(line: &str) -> u32 {
    let digit_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut transformed = String::new();
    let mut i = 0;

    while i < line.len() {
        if let Some((digit, word)) = digit_words
            .iter()
            .enumerate()
            .find(|(_, &word)| line[i..].starts_with(word))
        {
            transformed.push(((digit + 1) as u8 + b'0') as char);
            i += word.len() - 1;
        } else {
            transformed.push(line[i..].chars().next().unwrap());
            i += 1;
        }
    }

    question_a(&transformed)
}
