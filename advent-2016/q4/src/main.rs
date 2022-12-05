use std::{
    fs::File,
    io::{BufRead, BufReader},
};

extern crate regex;

fn main() {
    let part = 2;

    let file = File::open("src/input.txt").expect("File not found.");
    let reader = BufReader::new(file);

    let room_pattern = regex::Regex::new(r"^(.*)-(\d+)\[(.*)\]$").expect("could not compile regex");

    //count
    let mut count: u32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let parts = room_pattern.captures(&line).expect("could not match lini");

        let room_name = parts.get(1).expect("could not find room name").as_str();
        let computed_sum = name_checksum(room_name);
        let given_sum = parts.get(3).expect("could not find checksum").as_str();

        if computed_sum != given_sum {
            continue;
        }

        let sector_id: u32 = parts
            .get(2)
            .expect("count not find sector id")
            .as_str()
            .parse()
            .expect("could not parse sector id");

        if part == 2 {
            println!("{} {}", name_decrypt(room_name, sector_id), sector_id)
        }

        count += sector_id;
    }

    if part == 1 {
        println!("{}", count);
    }
}

fn name_checksum(room_name: &str) -> String {
    let mut sum = ['.'; 5];
    let mut nsum = 0;
    let mut counts = [0u32; 26];
    let ncounts = counts.len();

    for c in room_name.chars() {
        if c == '-' {
            continue;
        }
        counts[c as usize - b'a' as usize] += 1;
    }

    let mut m: u32 = *counts.iter().max().expect("could not find max count");
    if m == 0 {
        panic!("max is zero");
    }

    while nsum < 5 {
        let mut found_i = ncounts;

        for i in 0..ncounts {
            if counts[i] == m {
                found_i = i;
                break;
            }
        }
        if found_i == ncounts {
            if m > 1 {
                m -= 1;
                continue;
            }
            panic!("not enough chars for checksum")
        }

        sum[nsum] = (found_i as u8 + b'a') as char;
        counts[found_i] = 0;
        nsum += 1;
    }

    sum.iter().cloned().collect::<String>()
}

fn name_decrypt(room_name: &str, sector_id: u32) -> String {
    let mut result = Vec::<char>::new();

    for c in room_name.chars() {
        if c == '-' {
            result.push(' ');
        } else {
            let shift = c as u32 - b'a' as u32 + sector_id;
            result.push(((shift % 26) as u8 + b'a') as char)
        }
    }

    result.iter().cloned().collect::<String>()
}
