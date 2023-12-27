use std::{fs, time::Instant};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Tile {
    Empty,
    Mirror(bool),
    Splitter(bool),
}

#[derive(Clone)]
struct Board<T> {
    tiles: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Board<T> {
    fn new(tiles: Vec<T>, height: usize) -> Self {
        Board {
            width: tiles.len() / height,
            tiles,
            height,
        }
    }

    fn index(&self, point: Point) -> &T {
        &self.tiles[self.width * point.y + point.x]
    }

    fn index_mut(&mut self, point: Point) -> &mut T {
        &mut self.tiles[self.width * point.y + point.x]
    }

    fn go(&self, point: Point, direction: Direction) -> Option<Point> {
        match direction {
            Direction::North => self.up(point),
            Direction::East => self.right(point),
            Direction::South => self.down(point),
            Direction::West => self.left(point),
        }
    }

    fn up(&self, point: Point) -> Option<Point> {
        if point.y == 0 {
            return None;
        }

        Some(Point::new(point.x, point.y - 1))
    }
    fn down(&self, point: Point) -> Option<Point> {
        if point.y >= self.height - 1 {
            return None;
        }

        Some(Point::new(point.x, point.y + 1))
    }
    fn right(&self, point: Point) -> Option<Point> {
        if point.x >= self.width - 1 {
            return None;
        }

        Some(Point::new(point.x + 1, point.y))
    }
    fn left(&self, point: Point) -> Option<Point> {
        if point.x == 0 {
            return None;
        }

        Some(Point::new(point.x - 1, point.y))
    }
}

impl<T> std::ops::Index<Point> for Board<T> {
    fn index(&self, index: Point) -> &T {
        self.index(index)
    }

    type Output = T;
}
impl<T> std::ops::IndexMut<Point> for Board<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        self.index_mut(index)
    }
}

fn parse(input: &str) -> Board<Tile> {
    let tiles = input
        .bytes()
        .filter_map(|byte| match byte {
            b'\\' => Some(Tile::Mirror(false)),
            b'/' => Some(Tile::Mirror(true)),
            b'|' => Some(Tile::Splitter(false)),
            b'-' => Some(Tile::Splitter(true)),
            b'.' => Some(Tile::Empty),
            _ => None,
        })
        .collect();

    Board::new(tiles, input.lines().count())
}

fn solve(board: &Board<Tile>, start: (Point, Direction)) -> usize {
    let mut visited = vec![Board::new(vec![false; board.height * board.width], board.height); 5];

    let mut queue = Vec::new();
    queue.push(start);

    while let Some((mut point, mut direction)) = queue.pop() {
        loop {
            if visited[direction as usize][point] {
                break;
            }

            visited[4][point] = true;
            visited[direction as usize][point] = true;

            if let Tile::Mirror(mirror) = board[point] {
                if mirror {
                    direction = match direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::North,
                        Direction::South => Direction::West,
                        Direction::West => Direction::South,
                    }
                } else {
                    direction = match direction {
                        Direction::North => Direction::West,
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::North,
                    }
                }
            } else if let Tile::Splitter(split) = board[point] {
                if !split && (direction == Direction::East || direction == Direction::West) {
                    direction = Direction::North;
                    queue.push((point, Direction::South));
                } else if split && (direction == Direction::South || direction == Direction::North)
                {
                    direction = Direction::East;
                    queue.push((point, Direction::West));
                }
            }

            if let Some(next) = board.go(point, direction) {
                point = next;
            } else {
                break;
            }
        }
    }

    visited[4].tiles.iter().filter(|n| **n).count()
}

fn part_1(input: &str) -> usize {
    let map = parse(input);
    solve(&map, (Point::new(0, 0), Direction::East))
}

fn part_2(input: &str) -> usize {
    let map = parse(input);

    let mut max = 0;

    for i in 0..map.height {
        max = max.max(solve(&map, (Point::new(0, i), Direction::East)));
        max = max.max(solve(&map, (Point::new(map.width - 1, i), Direction::West)));
    }

    for i in 0..map.width {
        max = max.max(solve(&map, (Point::new(i, 0), Direction::South)));
        max = max.max(solve(
            &map,
            (Point::new(i, map.height - 1), Direction::North),
        ));
    }

    max
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file.");

    println!("--- Day 16: The Floor Will Be Lava ---");
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
