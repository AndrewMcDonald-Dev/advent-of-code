use std::fs;
fn main() {
    let input = fs::read_to_string("in.dat").expect("File not found or could not be read.");
    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}

fn part_a(input: &str) -> usize {
    const WINDOW_SIZE: usize = 4;
    input
        .as_bytes()
        .windows(WINDOW_SIZE)
        .position(|s| {
            (s[0] != s[1] && s[0] != s[2] && s[0] != s[3])
                && (s[1] != s[2] && s[1] != s[3])
                && (s[2] != s[3])
        })
        .unwrap()
        + WINDOW_SIZE
}

fn part_b(input: &str) -> usize {
    const WINDOW_SIZE: usize = 14;
    input
        .as_bytes()
        .windows(WINDOW_SIZE)
        .position(|s| {
            let mut seen = [false; 26]; //assumes only a-z
            for &e in s {
                let c = (e - b'a') as usize;
                if seen[c] {
                    return false;
                }
                seen[c] = true;
            }
            true
        })
        .unwrap()
        + WINDOW_SIZE
}
