use std::{collections::HashMap, fs, time::Instant};

type Input = Vec<(Vec<u8>, Vec<usize>)>;
type Map = HashMap<(usize, usize), u64>;

fn solve(input: &Input, repeat: usize) -> u64 {
    let mut result = 0;
    let mut bytes = Vec::new();
    let mut nums = Vec::new();
    let mut cache = HashMap::new();

    for (first, second) in input {
        // Handles Part 2
        for _ in 0..repeat {
            bytes.extend(first);
            bytes.push(b'?');
            nums.extend(second);
        }

        bytes.extend(first);
        bytes.push(b'.');
        nums.extend(second);

        let mut sum = 0;
        let mut ps = vec![0; nums.len()];

        for i in (1..nums.len()).rev() {
            sum += nums[i] + 1;
            ps[i - 1] = sum;
        }

        result += helper(&bytes, &nums, &ps, &mut cache);

        bytes.clear();
        nums.clear();
        cache.clear();
    }

    result
}

fn helper(slice: &[u8], nums: &[usize], ps: &[usize], cache: &mut Map) -> u64 {
    let key = (slice.len(), nums.len());
    if let Some(prev) = cache.get(&key) {
        return *prev;
    }

    if nums.is_empty() {
        let result = working(slice) as u64;
        cache.insert(key, result);
        return result;
    }

    let size = nums[0];
    let wiggle = slice.len() - ps[0] - size;
    let mut result = 0;

    for offset in 0..wiggle {
        if offset > 0 && slice[offset - 1] == b'#' {
            break;
        }
        if slice[offset + size] != b'#' && broken(&slice[offset..offset + size]) {
            result += helper(&slice[offset + size + 1..], &nums[1..], &ps[1..], cache);
        }
    }

    cache.insert(key, result);
    result
}

fn working(slice: &[u8]) -> bool {
    slice.iter().all(|&b| b == b'.' || b == b'?')
}

fn broken(slice: &[u8]) -> bool {
    slice.iter().all(|&b| b == b'#' || b == b'?')
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (prefix, suffix) = line.split_once(' ').unwrap();
            let first = prefix.as_bytes().to_vec();
            let second = suffix
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (first, second)
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file");
    let input = parse(&input);

    println!("{:-<13} Day 12: Hot Springs {:->13}", "", "");
    let before = Instant::now();
    println!(
        "Part 1: {:<14} | Elapsed Time: {:.2?}",
        solve(&input, 0),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "Part 2: {:<14} | Elapsed Time: {:.2?}",
        solve(&input, 4),
        before.elapsed()
    );
}
