use std::fs;

fn main() {
    let input = fs::read_to_string("in.dat").expect("File not found or could not be read.");
    question_a(&input);
    question_b(&input)
}

fn question_a(input: &str) {
    println!(
        "A: {}",
        input
            .split("\r\n\r\n")
            .map(|elf| elf
                .lines()
                .map(|item| item.trim().parse::<u32>().expect("Could not parse."))
                .sum::<u32>())
            .max()
            .unwrap()
    );
}

fn question_b(input: &str) {
    let mut top = [0; 3];

    for calories in input.split("\r\n\r\n").map(|elf| {
        elf.lines()
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
    }) {
        let top_min = top.iter_mut().min().unwrap();
        if calories > *top_min {
            *top_min = calories;
        }
    }

    println!("B: {}", top.iter().sum::<u32>())
}
