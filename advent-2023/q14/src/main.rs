use core::fmt;
use std::{fs, time::Instant};

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    grid: Vec<Vec<Space>>,
}

impl Grid {
    fn get(&self, point: (usize, usize)) -> &Space {
        &self.grid[point.1][point.0]
    }
    fn iter(&self) -> impl Iterator<Item = &Vec<Space>> {
        self.grid.iter()
    }
    fn set(&mut self, new: Space, point: (usize, usize)) {
        self.grid[point.1][point.0] = new;
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.clone() {
            for e in row {
                write!(f, "{}", e)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Space {
    Round,
    Square,
    Empty,
}

impl Space {
    fn cvt(c: char) -> Self {
        match c {
            '#' => Self::Square,
            'O' => Self::Round,
            '.' => Self::Empty,
            _ => panic!("Bad character input"),
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Space::Round => 'O',
            Space::Square => '#',
            Space::Empty => '.',
        };

        write!(f, "{}", c)
    }
}

fn parse(input: String) -> Grid {
    Grid {
        grid: input
            .lines()
            .map(|line| line.chars().map(Space::cvt).collect())
            .collect(),
    }
}

fn part_1(mut input: Grid) -> usize {
    for y in 0..input.grid.len() {
        for x in 0..input.grid[0].len() {
            if let Space::Round = input.grid[y][x] {
                let mut k = y;

                while k > 0 && input.grid[k - 1][x] == Space::Empty {
                    k -= 1;
                }

                input.grid[y][x] = Space::Empty;
                input.grid[k][x] = Space::Round;
            }
        }
    }

    calculate_load(&input)
}

fn calculate_load(tilted: &Grid) -> usize {
    let height = tilted.height();
    tilted
        .iter()
        .enumerate()
        .map(|(index, row)| {
            row.iter()
                .map(|space| {
                    if let Space::Round = space {
                        height - index
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}
//
// fn tilt_north_one(input: &Grid, point: (usize, usize)) -> (usize, usize) {
//     let mut point = point;
//     while point.1 != 0 {
//         if let Space::Empty = input.get((point.0, point.1 - 1)) {
//             point = (point.0, point.1 - 1);
//         } else {
//             break;
//         }
//     }
//     point
// }
//
// fn tilt_west_one(input: &Grid, point: (usize, usize)) -> (usize, usize) {
//     let mut point = point;
//     while point.0 != 0 {
//         if let Space::Empty = input.get((point.0 - 1, point.1)) {
//             point = (point.0 - 1, point.1);
//         } else {
//             break;
//         }
//     }
//     point
// }
//
// fn tilt_south_one(input: &Grid, point: (usize, usize), max_y: usize) -> (usize, usize) {
//     let mut point = point;
//     while point.1 != max_y {
//         if let Space::Empty = input.get((point.0, point.1 + 1)) {
//             point = (point.0, point.1 + 1);
//         } else {
//             break;
//         }
//     }
//     point
// }
//
// fn tilt_east_one(input: &Grid, point: (usize, usize), max_x: usize) -> (usize, usize) {
//     let mut point = point;
//     while point.0 != max_x {
//         if let Space::Empty = input.get((point.0 + 1, point.1)) {
//             point = (point.0 + 1, point.1);
//         } else {
//             break;
//         }
//     }
//     point
// }
//
//
// fn part_2(input: &Grid, iterations: usize) -> usize {
//     let mut input = input.clone();
//     let max_x = input.grid[0].len() - 1;
//     let max_y = input.grid.len() - 1;
//     for n in 0..iterations {
//         // North
//         let mut north_grid = input.clone();
//         for (y, list) in input.iter().enumerate() {
//             for (x, _) in list.iter().enumerate() {
//                 if let Space::Round = input.get((x, y)) {
//                     let new = tilt_north_one(&north_grid, (x, y));
//                     north_grid.set(Space::Round, new);
//                     if new.0 != x || new.1 != y {
//                         north_grid.set(Space::Empty, (x, y));
//                     }
//                 }
//             }
//         }
//
//         // West
//         let mut west_grid = north_grid.clone();
//         for (y, list) in north_grid.iter().enumerate() {
//             for (x, _) in list.iter().enumerate() {
//                 if let Space::Round = north_grid.get((x, y)) {
//                     let new = tilt_west_one(&west_grid, (x, y));
//                     west_grid.set(Space::Round, new);
//                     if new.0 != x || new.1 != y {
//                         west_grid.set(Space::Empty, (x, y));
//                     }
//                 }
//             }
//         }
//
//         // South
//         let mut south_grid = west_grid.clone();
//         for (y, list) in west_grid.iter().enumerate() {
//             for (x, _) in list.iter().enumerate() {
//                 if let Space::Round = west_grid.get((x, y)) {
//                     let new = tilt_south_one(&south_grid, (x, y), max_y);
//                     south_grid.set(Space::Round, new);
//                     if new.0 != x || new.1 != y {
//                         south_grid.set(Space::Empty, (x, y));
//                     }
//                 }
//             }
//         }
//
//         // East
//         let mut east_grid = south_grid.clone();
//         for (y, list) in south_grid.iter().enumerate() {
//             for (x, _) in list.iter().enumerate() {
//                 if let Space::Round = south_grid.get((x, y)) {
//                     let new = tilt_east_one(&east_grid, (x, y), max_x);
//                     east_grid.set(Space::Round, new);
//                     if new.0 != x || new.1 != y {
//                         east_grid.set(Space::Empty, (x, y));
//                     }
//                 }
//             }
//         }
//         if n % 1000000 == 0 {
//             println!("Progress: {}%", n % 1000000);
//         }
//         input = east_grid.clone();
//     }
//
//     calculate_load(&input)
// }

fn part_2(mut input: Grid, iterations: usize) -> usize {
    for n in 0..iterations {
        // North
        for y in 0..input.grid.len() {
            for x in 0..input.grid[0].len() {
                if let Space::Round = input.grid[y][x] {
                    let mut k = y;

                    while k > 0 && input.grid[k - 1][x] == Space::Empty {
                        k -= 1;
                    }

                    input.grid[y][x] = Space::Empty;
                    input.grid[k][x] = Space::Round;
                }
            }
        }

        // West
        for y in 0..input.grid.len() {
            for x in 0..input.grid[0].len() {
                if let Space::Round = input.grid[y][x] {
                    let mut k = x;

                    while k > 0 && input.grid[y][k - 1] == Space::Empty {
                        k -= 1;
                    }

                    input.grid[y][x] = Space::Empty;
                    input.grid[y][k] = Space::Round;
                }
            }
        }

        // South
        for y in 0..input.grid.len() {
            for x in 0..input.grid[0].len() {
                if let Space::Round = input.grid[y][x] {
                    let mut k = y;

                    while k < input.grid.len() - 1 && input.grid[k + 1][x] == Space::Empty {
                        k += 1;
                    }

                    input.grid[y][x] = Space::Empty;
                    input.grid[k][x] = Space::Round;
                }
            }
        }

        // East
        for y in 0..input.grid.len() {
            for x in 0..input.grid[0].len() {
                if let Space::Round = input.grid[y][x] {
                    let mut k = x;

                    while k < input.grid[0].len() - 1 && input.grid[y][k + 1] == Space::Empty {
                        k += 1;
                    }

                    input.grid[y][x] = Space::Empty;
                    input.grid[y][k] = Space::Round;
                }
            }
        }
        if n % 1000000 == 0 {
            println!("Progress: {}%", n % 1000000);
        }
    }

    calculate_load(&input)
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file");

    println!("{:-<6} Day 14: Parabolic Reflector Dish {:->6}", "", "");
    let before = Instant::now();
    println!(
        "Part 1: {:<14} | Elapsed Time: {:.2?}",
        part_1(parse(input.clone())),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "Part 2: {:<14} | Elapsed Time: {:.2?}",
        part_2(parse(input), 1000000000),
        before.elapsed()
    );
}
