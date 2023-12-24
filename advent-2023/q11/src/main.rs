use std::{fs, time::Instant};

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

fn part_1(input: &str) -> usize {
    solve(&parse(input, 2))
}
fn part_2(input: &str) -> usize {
    solve(&parse(input, 1000000))
}

fn solve(galaxies: &Vec<Point>) -> usize {
    let mut sum = 0;

    for i in 0..galaxies.len() {
        let a = &galaxies[i];
        for b in &galaxies[i + 1..] {
            sum += a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
        }
    }

    sum
}

fn parse(input: &str, expansion: usize) -> Vec<Point> {
    let mut galaxies: Vec<Point> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut x = 0;
        for byte in line.bytes() {
            match byte {
                b'#' => {
                    galaxies.push(Point::new(x, y));
                    x += 1;
                }
                b'.' => {
                    x += 1;
                }
                _ => {}
            }
        }
    }

    let mut sum_expansion = 0;
    let mut last_y = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.y != last_y {
            sum_expansion += (galaxy.y - last_y - 1) * (expansion - 1);
            last_y = galaxy.y;
        }
        galaxy.y += sum_expansion;
    }

    galaxies.sort_by_key(|galaxy| galaxy.x);

    sum_expansion = 0;
    let mut last_x = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.x != last_x {
            sum_expansion += (galaxy.x - last_x - 1) * (expansion - 1);
            last_x = galaxy.x;
        }

        galaxy.x += sum_expansion
    }

    galaxies
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file.");

    println!("--- Day 4: Scratchcards ---");
    let now = Instant::now();
    println!(
        "Part 1: {} | Elapsed: {:.2?}",
        part_1(&input),
        now.elapsed()
    );
    let now = Instant::now();
    println!(
        "Part 2: {} | Elapsed: {:.2?}",
        part_2(&input),
        now.elapsed()
    );
}
