use std::fs;

fn main() {
    let input = fs::read_to_string("in.dat").expect("File not found or could not be read.");
    question_a(&input);
    question_b(&input)
}

fn section_range_pairs(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let ranges = line.split(',').collect::<Vec<&str>>();
            (get_range(ranges[0]), get_range(ranges[1]))
        })
        .collect()
}

fn get_range(input: &str) -> (i32, i32) {
    let range_ends = input.split('-').collect::<Vec<&str>>();
    (
        range_ends[0].parse::<i32>().unwrap(),
        range_ends[1].parse::<i32>().unwrap(),
    )
}

fn is_contained(left: (i32, i32), right: (i32, i32)) -> bool {
    left.0 >= right.0 && left.1 <= right.1
}

fn question_a(input: &str) {
    let result: i32 = section_range_pairs(input)
        .iter()
        .map(|pair| {
            if is_contained(pair.0, pair.1) || is_contained(pair.1, pair.0) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{result}");
}

fn question_b(input: &str) {
    let result: i32 = section_range_pairs(input)
        .iter()
        .map(|pair| if is_overlapping(pair.0, pair.1) { 1 } else { 0 })
        .sum();

    println!("{result}")
}

fn is_overlapping(left: (i32, i32), right: (i32, i32)) -> bool {
    (left.0 >= right.0 && left.0 <= right.1)
        || (left.1 >= right.0 && left.1 <= right.1)
        || is_contained(right, left)
}
