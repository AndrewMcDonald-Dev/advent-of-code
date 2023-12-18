use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
    time::Instant,
};

type Input = Vec<Vec<usize>>;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn turn_right(&self) -> Self {
        Coordinates {
            x: self.y,
            y: self.x,
        }
    }

    fn turn_left(&self) -> Self {
        Coordinates {
            x: -self.y,
            y: -self.x,
        }
    }

    fn add(&self, other: Coordinates) -> Self {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<(i32, i32)> for Coordinates {
    fn from(value: (i32, i32)) -> Self {
        Coordinates {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    coordinates: Coordinates,
    direction: Coordinates,
    heat_loss: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| other.direction.cmp(&self.direction))
            .then_with(|| self.coordinates.x.cmp(&other.coordinates.x))
            .then_with(|| self.coordinates.y.cmp(&other.coordinates.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &[Vec<usize>], min_consecutive: usize, max_consecutive: usize) -> Option<usize> {
    let max_x = grid[0].len() as i32 - 1;
    let max_y = grid.len() as i32 - 1;

    let mut visited: HashMap<(Coordinates, Coordinates), usize> = HashMap::new();

    let mut nodes: BinaryHeap<State> = BinaryHeap::new();

    nodes.push(State {
        coordinates: (1, 0).into(),
        direction: (1, 0).into(),
        heat_loss: 0,
    });

    nodes.push(State {
        coordinates: (0, 1).into(),
        direction: (0, 1).into(),
        heat_loss: 0,
    });

    let mut min_heat_loss: Option<usize> = None;

    while let Some(State {
        coordinates,
        direction,
        heat_loss,
    }) = nodes.pop()
    {
        let prev_heat_loss = visited
            .entry((coordinates, direction))
            .or_insert(heat_loss + 1);

        if *prev_heat_loss > heat_loss {
            *prev_heat_loss = heat_loss;
        } else {
            continue;
        }

        let mut heat_loss = heat_loss;
        let mut coordinates = coordinates;

        for step in 0..max_consecutive as i32 {
            if coordinates.x < 0
                || coordinates.x > max_x
                || coordinates.y < 0
                || coordinates.y > max_y
            {
                break;
            }

            heat_loss += grid[coordinates.y as usize][coordinates.x as usize];

            if coordinates == (max_x, max_y).into() {
                if min_heat_loss.unwrap_or(heat_loss + 1) > heat_loss
                    && step >= min_consecutive as i32 - 1
                {
                    min_heat_loss = Some(heat_loss);
                }

                break;
            }

            if step >= min_consecutive as i32 - 1 {
                nodes.push(State {
                    coordinates: coordinates.add(direction.turn_left()),
                    direction: direction.turn_left(),
                    heat_loss,
                });
                nodes.push(State {
                    coordinates: coordinates.add(direction.turn_right()),
                    direction: direction.turn_right(),
                    heat_loss,
                });
            }

            coordinates = coordinates.add(direction);
        }
    }

    min_heat_loss
}

fn part_1(grid: &Input) -> usize {
    dijkstra(grid, 0, 3).unwrap_or(0)
}

fn part_2(grid: &Input) -> usize {
    dijkstra(grid, 4, 10).unwrap_or(0)
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = fs::read_to_string("in.dat").unwrap();
    let input = parse(&input);

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
