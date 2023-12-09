use std::fs;

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file.");
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> i64 {
    solve(input, true)
}
fn part_2(input: &str) -> i64 {
    solve(input, false)
}

fn solve(input: &str, p1: bool) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            calc(&nums, p1)
        })
        .sum()
}

fn calc(nums: &[i64], p1: bool) -> i64 {
    let delta = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(l, r)| r - l)
        .collect::<Vec<_>>();

    if p1 {
        if delta.iter().all(|n| *n == 0) {
            *nums.last().unwrap()
        } else {
            nums.last().unwrap() + calc(&delta, p1)
        }
    } else if delta.iter().all(|n| *n == 0) {
        *nums.first().unwrap()
    } else {
        nums.first().unwrap() - calc(&delta, p1)
    }
}
