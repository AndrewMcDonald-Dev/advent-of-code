use std::{fs, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Round,
    Square,
    Empty,
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Square,
                    'O' => Tile::Round,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}

fn slide_north(grid: &mut [Vec<Tile>]) {
    for col in 0..grid[0].len() {
        let mut empty_or_round_row = 0;
        for row in 0..grid.len() {
            let curr = grid[row][col];
            match curr {
                Tile::Square => empty_or_round_row = row + 1,
                Tile::Round => {
                    // swap the current tile with the empty_or_round one
                    let replace_with = std::mem::replace(&mut grid[empty_or_round_row][col], curr);
                    let _ = std::mem::replace(&mut grid[row][col], replace_with);
                    empty_or_round_row += 1;
                }
                Tile::Empty => (),
            }
        }
    }
}

fn weight(grid: &[Vec<Tile>]) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            let round_rocks = row.iter().filter(|tile| **tile == Tile::Round).count();
            round_rocks * (i + 1)
        })
        .sum()
}

// rotate 90 degrees clockwise: (x, y) -> (y, -x)
fn clockwise(grid: &mut Vec<Vec<Tile>>) {
    let size = grid.len();
    let mut rotated = vec![vec![Tile::Empty; size]; size];
    (0..size).for_each(|row| {
        (0..size).for_each(|col| {
            rotated[col][size - 1 - row] = grid[row][col];
        });
    });
    *grid = rotated;
}

#[inline]
fn cycle(grid: &mut Vec<Vec<Tile>>) {
    for _ in 0..4 {
        slide_north(grid);
        clockwise(grid);
    }
}

fn part_1(mut grid: Vec<Vec<Tile>>) -> usize {
    slide_north(&mut grid);
    weight(&grid)
}

fn part_2(mut grid: Vec<Vec<Tile>>) -> usize {
    let mut seen = vec![grid.clone()];

    loop {
        cycle(&mut grid);
        // check if the cycled map has already been seen
        if let Some(idx) = seen.iter().position(|x| x == &grid) {
            // figure out length of cycle (watch out: a cycle might only start after a number of steps)
            let cycle_len = seen.len() - idx;
            // use cycle length to figure out the index of the final step in the seen list
            let final_idx = idx + (1_000_000_000 - idx) % cycle_len;
            return weight(&seen[final_idx]);
        }
        seen.push(grid.clone());
    }
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file");

    println!("{:-<6} Day 14: Parabolic Reflector Dish {:->6}", "", "");
    let before = Instant::now();
    println!(
        "Part 1: {:<14} | Elapsed Time: {:.2?}",
        part_1(parse(&input)),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "Part 2: {:<14} | Elapsed Time: {:.2?}",
        part_2(parse(&input)),
        before.elapsed()
    );
}
