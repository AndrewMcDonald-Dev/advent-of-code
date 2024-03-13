use regex::Regex;
use std::{fs, time::Instant};

fn part_1(input: &str) -> u32 {
    input.split(',').map(|s| hash(&mut s.trim().chars())).sum()
}

pub fn hash<I: Iterator<Item = char>>(input: &mut I) -> u32 {
    input.fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

fn part_2(data: &str) -> usize {
    const VAL: Vec<(String, usize)> = vec![];
    let mut boxes: [Vec<(String, usize)>; 256] = [VAL; 256];

    let pattern_re: Regex = Regex::new(r"(\w+)([-=])(\d)?").unwrap();

    data.split(',').for_each(|ins| {
        let caps = pattern_re.captures(ins).unwrap();

        let lens = caps.get(1).unwrap().as_str();
        let sign = caps.get(2).unwrap().as_str();

        let box_index = hash(&mut lens.chars());

        let valid_box = &mut boxes[box_index as usize];
        if sign == "=" {
            let power: usize = caps.get(3).unwrap().as_str().parse().unwrap();
            if let Some(found_index) = valid_box
                .iter()
                .position(|(lens_in_box, _)| lens_in_box == lens)
            {
                valid_box[found_index] = (lens.to_string(), power);
            } else {
                valid_box.push((lens.to_string(), power));
            }
        } else {
            valid_box.retain(|(lens_in_box, _)| lens_in_box != lens);
        }
    });

    (0..256usize)
        .map(|box_idx| {
            boxes[box_idx]
                .iter()
                .enumerate()
                .map(|(slot, (_, power))| (slot + 1) * *power * (box_idx + 1))
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file");

    println!("{:-<13} Day 15: Lens Library {:->13}", "", "");
    let before = Instant::now();
    println!(
        "Part 1: {:<14} | Elapsed Time: {:.2?}",
        part_1(&input),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "Part 2: {:<14} | Elapsed Time: {:.2?}",
        part_2(&input),
        before.elapsed()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash_test() {
        assert_eq!(30, crate::hash(&mut "rn=1".chars()));
    }
}
