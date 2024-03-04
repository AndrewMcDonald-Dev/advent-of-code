use std::{fs, time::Instant};

fn test_row(grid: &[Vec<char>], index: usize) -> bool {
    let height = usize::min(index + 1, grid.len() - index - 1);

    (0..height).all(|i| {
        let row_above = &grid[index - i];
        let row_below = &grid[index + i + 1];

        row_above == row_below
    })
}

fn test_col(grid: &[Vec<char>], index: usize) -> bool {
    let width = usize::min(index + 1, grid[0].len() - index - 1);

    (0..width).all(|i| {
        let col_left = grid.iter().map(|row| row[index - i]);
        let col_right = grid.iter().map(|row| row[index + i + 1]);

        col_left.eq(col_right)
    })
}

fn process_grid(grid: &[Vec<char>]) -> Option<usize> {
    let row = (0..grid.len() - 1).find_map(|i| {
        if test_row(grid, i) {
            Some(100 * (i + 1))
        } else {
            None
        }
    });
    let col =
        (0..grid[0].len() - 1).find_map(|i| if test_col(grid, i) { Some(i + 1) } else { None });

    row.or(col)
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|grid| {
            let grid = grid
                .lines()
                .map(|row| row.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            process_grid(&grid).unwrap()
        })
        .sum()
}

fn diff_row(grid: &[Vec<char>], index: usize) -> usize {
    let height = usize::min(index + 1, grid.len() - index - 1);

    (0..height)
        .map(|i| {
            let row_above = &grid[index - i];
            let row_below = &grid[index + i + 1];

            Iterator::zip(row_above.iter(), row_below.iter())
                .filter(|(a, b)| a != b)
                .count()
        })
        .sum()
}

fn diff_col(grid: &[Vec<char>], index: usize) -> usize {
    let width = usize::min(index + 1, grid[0].len() - index - 1);

    (0..width)
        .map(|i| {
            let col_left = grid.iter().map(|row| row[index - i]);
            let col_right = grid.iter().map(|row| row[index + i + 1]);

            Iterator::zip(col_left, col_right)
                .filter(|(a, b)| a != b)
                .count()
        })
        .sum()
}

fn process_grid_2(grid: &[Vec<char>]) -> Option<usize> {
    let row = (0..grid.len() - 1).find_map(|i| {
        if diff_row(grid, i) == 1 {
            Some(100 * (i + 1))
        } else {
            None
        }
    });
    let col = (0..grid[0].len() - 1).find_map(|i| {
        if diff_col(grid, i) == 1 {
            Some(i + 1)
        } else {
            None
        }
    });

    row.or(col)
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|grid| {
            let grid = grid
                .lines()
                .map(|row| row.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            process_grid_2(&grid).unwrap()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("in.dat").unwrap();
    // let input = parse(&input);

    println!("--- Day 17: Clumsy Crucible ---");
    let now = Instant::now();
    println!(
        "Part 1: {:4} | Elapsed: {:.2?}",
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
