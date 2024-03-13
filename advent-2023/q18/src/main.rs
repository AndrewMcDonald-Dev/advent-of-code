use regex::Regex;
use std::{fs, time::Instant};

fn read_directions(text: &str) -> Vec<(char, i64)> {
    let regex = Regex::new(r"(?m)^([RLDU]) ([[:digit:]]+)").unwrap();
    regex
        .captures_iter(text)
        .map(|cap| {
            let (_, [digit, number]) = cap.extract();
            (digit.chars().next().unwrap(), number.parse().unwrap())
        })
        .collect()
}

fn read_directions_2(text: &str) -> Vec<(char, i64)> {
    let regex = Regex::new(r"(?m)\(\#([[:xdigit:]]{5})([0-3])\)$").unwrap();
    regex
        .captures_iter(text)
        .map(|cap| {
            let (_, [hexstr, d]) = cap.extract();
            let d_int = usize::from_str_radix(d, 16).unwrap();
            let dir = ['R', 'D', 'L', 'U'][d_int];
            let hex = i64::from_str_radix(hexstr, 16).unwrap();
            (dir, hex)
        })
        .collect()
}

fn get_area(dirs: &[(char, i64)]) -> i64 {
    let (perimeter, area, _) = dirs
        .iter()
        .fold((0, 0, (0, 0)), |(p, a, (y, x)), (d, l)| match d {
            'R' => (p + l, a, (y, x + l)),
            'L' => (p + l, a, (y, x - l)),
            'D' => (p + l, a + x * l, (y + l, x)),
            'U' => (p + l, a - x * l, (y - l, x)),
            _ => panic!("Got {d}, expected R, L, D, or U"),
        });
    area + perimeter / 2 + 1
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file");

    println!("{:-<13} Day 15: Lens Library {:->13}", "", "");
    let before = Instant::now();
    println!(
        "Part 1: {:<14} | Elapsed Time: {:.2?}",
        get_area(&read_directions(&input)),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "Part 2: {:<14} | Elapsed Time: {:.2?}",
        get_area(&read_directions_2(&input)),
        before.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_first() {
        let result = get_area(&read_directions(INPUT));
        assert_eq!(result, 62);
    }

    #[test]
    fn test_second() {
        let result = get_area(&read_directions_2(INPUT));
        assert_eq!(result, 952_408_144_115);
    }
}
