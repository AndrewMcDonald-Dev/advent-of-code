use core::fmt;
use std::{
    collections::{HashMap, VecDeque},
    fs,
    time::Instant,
};

// General structure for solution:
// point x is a '.' and a 'O' or 'S' is next to it
// mark that spot on a fresh graph as an 'O'.
// The fresh graph is created at the start of every loop.
// The fresh graph is the last iteration with all 'O' and 'S' removed.
// Iterate 64 times for answer.
//
#[derive(PartialEq, Clone)]
enum Step {
    Start,
    Empty,
    Rock,
    Step,
}

impl Step {
    fn map(character: char) -> Self {
        match character {
            'S' => Step::Start,
            '.' => Step::Empty,
            '#' => Step::Rock,
            'O' => Step::Step,
            _ => panic!("Bad input"),
        }
    }

    fn check_move(step: &Step) -> bool {
        *step == Step::Start || *step == Step::Step
    }

    fn check_move_p2(step: &Step) -> bool {
        *step != Step::Rock
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Step::Start => write!(f, "S"),
            Step::Empty => write!(f, "."),
            Step::Rock => write!(f, "#"),
            Step::Step => write!(f, "O"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point(usize, usize);

struct Plot {
    plot: Vec<Vec<Step>>,
    start: Option<Point>,
}

impl Plot {
    fn parse(input: &str) -> Self {
        let plot = input
            .lines()
            .map(|line| line.chars().map(Step::map).collect())
            .collect();

        Plot { plot, start: None }
    }

    fn create_fresh_plot(&self) -> Plot {
        let plot = self
            .plot
            .clone()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|step| {
                        if *step == Step::Step || *step == Step::Start {
                            Step::Empty
                        } else {
                            step.clone()
                        }
                    })
                    .collect()
            })
            .collect();
        Plot { plot, start: None }
    }

    fn count_steps(&self) -> u32 {
        self.plot
            .iter()
            .map(|row| {
                row.iter()
                    .map(|step| if *step == Step::Step { 1 } else { 0 })
                    .sum::<u32>()
            })
            .sum()
    }

    fn find_start(&mut self) {
        for (y, row) in self.plot.iter().enumerate() {
            for (x, step) in row.iter().enumerate() {
                if *step == Step::Start {
                    self.start = Some(Point(x, y));
                }
            }
        }
    }

    fn calculate_distances(&self) -> HashMap<Point, i32> {
        let mut distances = HashMap::new();
        let mut frontier = VecDeque::new();
        frontier.push_back((self.start.unwrap(), 0));

        while let Some((p, dist)) = frontier.pop_front() {
            if distances.contains_key(&p) {
                continue;
            }

            distances.insert(p, dist);

            let max_x = self.plot[0].len();
            let max_y = self.plot.len();

            let x = p.0;
            let y = p.1;

            if x + 1 < max_x && Step::check_move_p2(&self.plot[y][x + 1]) {
                frontier.push_back((Point(x + 1, y), dist + 1))
            }
            if x > 0 && Step::check_move_p2(&self.plot[y][x - 1]) {
                frontier.push_back((Point(x - 1, y), dist + 1))
            }
            if y + 1 < max_y && Step::check_move_p2(&self.plot[y + 1][x]) {
                frontier.push_back((Point(x, y + 1), dist + 1))
            }
            if y > 0 && Step::check_move_p2(&self.plot[y - 1][x]) {
                frontier.push_back((Point(x, y - 1), dist + 1))
            }
        }

        distances
    }
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self
            .plot
            .iter()
            .map(|row| {
                row.iter().fold("\n".to_string(), |string, step| {
                    format!("{}{}", string, step)
                })
            })
            .collect::<String>();

        write!(f, "{}", output)
    }
}

fn part_1(input: &str) -> u32 {
    // Parse input into 2dvec of enums
    let mut plot = Plot::parse(input);

    let max_x = plot.plot[0].len();
    let max_y = plot.plot.len();

    for _ in 0..64 {
        // create fresh 2dvec
        let mut fresh_plot = plot.create_fresh_plot();

        // iterate over fresh 2dvec and check last iteration for adjacent steps
        for (y, row) in fresh_plot.plot.iter_mut().enumerate() {
            for (x, step) in row.iter_mut().enumerate() {
                if *step == Step::Empty
                    && ((x + 1 < max_x && Step::check_move(&plot.plot[y][x + 1]))
                        || (x > 0 && Step::check_move(&plot.plot[y][x - 1]))
                        || (y + 1 < max_y && Step::check_move(&plot.plot[y + 1][x]))
                        || (y > 0 && Step::check_move(&plot.plot[y - 1][x])))
                {
                    *step = Step::Step;
                }
            }
        }

        // fresh 2dvec replaces last vec iteration repeats
        plot = fresh_plot;
    }

    // Count steps in plot and return
    plot.count_steps()
}

fn part_2(input: &str) -> usize {
    // Parse input into 2dvec of enums
    let mut plot = Plot::parse(input);

    // Find start
    plot.find_start();

    // Calculate the distances from each point to the start
    let distances = plot.calculate_distances();

    // Get a count of all odd and even blocks as well as
    // edges of odds and evens
    let (odd, even, odd_edges, even_edges) = distances.iter().fold(
        (0_usize, 0_usize, 0_usize, 0_usize),
        |(odd, even, odd_edges, even_edges), (_, v)| {
            if *v % 2 == 1 && *v > 65 {
                (odd + 1, even, odd_edges + 1, even_edges)
            } else if *v % 2 == 1 {
                (odd + 1, even, odd_edges, even_edges)
            } else if *v % 2 == 0 && *v > 65 {
                (odd, even + 1, odd_edges, even_edges + 1)
            } else {
                (odd, even + 1, odd_edges, even_edges)
            }
        },
    );

    // 26501365 % 131 = 65
    // (26501365 - 65) / 131 = 202300
    // Magic number
    let count = 202300;

    // The total odd and total even
    let total_odd = odd * (count + 1) * (count + 1);
    let total_even = even * (count * count);

    // total odd edges and total even edges
    let total_odd_edges = odd_edges * (count + 1);
    let total_even_edges = count * even_edges;

    // Diferrence of Odd/Even Blocks and Odd/Even edges.
    total_odd + total_even - total_odd_edges + total_even_edges
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file");

    println!("{:-<10} Day 21: Step Counter {:->10}", "", "");
    let before = Instant::now();
    println!(
        "Part 1: {:<15} | Elapsed Time: {:.2?}",
        part_1(&input),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "Part 2: {:<15} | Elapsed Time: {:.2?}",
        part_2(&input),
        before.elapsed()
    );
}
