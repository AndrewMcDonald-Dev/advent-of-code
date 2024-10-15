use core::panic;
use std::mem;
use std::{collections::VecDeque, fs, ops::ControlFlow, time::Instant};

pub struct VecMap<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
}

impl<K, V> VecMap<K, V> {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }
}

// impl Default for VecMap
impl<K, V> Default for VecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq, V> VecMap<K, V> {
    pub fn index_of_key(&self, k: &K) -> Option<usize> {
        self.keys.iter().position(|key| key == k)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        match self.keys.iter().position(|key| key == &k) {
            Some(i) => {
                let mut v = v;
                mem::swap(&mut v, &mut self.values[i]);
                Some(v)
            }
            None => {
                self.keys.push(k);
                self.values.push(v);
                None
            }
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.keys
            .iter()
            .position(|key| key == k)
            .map(|i| &self.values[i])
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.keys.iter().any(|key| key == k)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq)]
enum Land {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Clone, PartialEq, Copy, Eq, Hash)]
struct Position(usize, usize);

struct Grid(Vec<Vec<Land>>);

impl Grid {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Land::Path,
                        '#' => Land::Forest,
                        '^' => Land::Slope(Direction::North),
                        '<' => Land::Slope(Direction::West),
                        '>' => Land::Slope(Direction::East),
                        'v' => Land::Slope(Direction::South),
                        _ => panic!("Bad input"),
                    })
                    .collect()
            })
            .collect();

        Grid(grid)
    }

    fn inbounds(pos: Position, extents: Position) -> bool {
        pos.0 < extents.0 && pos.1 < extents.1
    }

    fn valid_step(land: Land) -> bool {
        land != Land::Forest
    }

    fn go_in_direction(
        &self,
        mut path: Vec<Position>,
        path_end: Position,
        direction: &Direction,
        extents: Position,
        queue: &mut VecDeque<Vec<Position>>,
    ) {
        let neighbor = match direction {
            Direction::North => Position(path_end.0 - 1, path_end.1),
            Direction::South => Position(path_end.0 + 1, path_end.1),
            Direction::West => Position(path_end.0, path_end.1 - 1),
            Direction::East => Position(path_end.0, path_end.1 + 1),
        };
        if !path.contains(&neighbor)
            && Grid::inbounds(neighbor, extents)
            && Grid::valid_step(self.0[neighbor.0][neighbor.1])
        {
            path.push(neighbor);
            queue.push_back(path.clone());
        }
    }

    fn walk_grid(grid: Grid, start: Position, end: Position) -> usize {
        let mut queue = VecDeque::new();
        let mut paths: Vec<Vec<Position>> = vec![];
        let extents = Position(grid.0.len(), grid.0[0].len());
        queue.push_back(vec![start]);

        while !queue.is_empty() {
            let path = queue.pop_front().unwrap().clone();
            let path_end = *path.iter().last().unwrap();
            if path_end == end {
                paths.push(path);
            } else {
                let object = &grid.0[path_end.0][path_end.1];
                match object {
                    Land::Path => {
                        grid.go_in_direction(
                            path.clone(),
                            path_end,
                            &Direction::North,
                            extents,
                            &mut queue,
                        );
                        grid.go_in_direction(
                            path.clone(),
                            path_end,
                            &Direction::South,
                            extents,
                            &mut queue,
                        );
                        grid.go_in_direction(
                            path.clone(),
                            path_end,
                            &Direction::West,
                            extents,
                            &mut queue,
                        );
                        grid.go_in_direction(
                            path.clone(),
                            path_end,
                            &Direction::East,
                            extents,
                            &mut queue,
                        );
                    }
                    Land::Forest => {
                        panic!("Made bad step")
                    }
                    Land::Slope(direction) => {
                        grid.go_in_direction(path, path_end, direction, extents, &mut queue)
                    }
                }
            }
        }

        paths.iter().map(|x| x.len()).max().unwrap() - 1
    }

    fn dfs(
        start: usize,
        end: usize,
        seen: &mut Vec<bool>,
        neighbors: &Vec<Vec<(usize, usize)>>,
    ) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        seen[start] = true;
        let longest_path = neighbors[start]
            .iter()
            .filter_map(|(cost, np)| {
                if seen[*np] {
                    None
                } else {
                    Self::dfs(*np, end, seen, neighbors).map(|ans| ans + *cost)
                }
            })
            .max();
        seen[start] = false;
        longest_path
    }

    fn neighbors(position: Position, grid: &Grid) -> impl Iterator<Item = Position> + '_ {
        let dpos = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        (0..4).filter_map(move |dir| {
            let Position(i, j) = position;
            if dir == 0 && i == 0
                || dir == 1 && j == grid.0[0].len() - 1
                || dir == 2 && i == grid.0.len() - 1
                || dir == 3 && j == 0
            {
                return None;
            }
            let (di, dj) = dpos[dir];
            let np = Position(
                (position.0 as i32 + di) as usize,
                (position.1 as i32 + dj) as usize,
            );
            match (grid.0[np.0][np.1], dir) {
                (Land::Forest, _) => None,
                _ => Some(np),
            }
        })
    }

    fn precalc_neighbors(start: Position, grid: &Grid) -> VecMap<Position, Vec<(usize, usize)>> {
        let mut stack = vec![start];
        let mut h = VecMap::new();
        while let Some(position) = stack.pop() {
            if h.contains_key(&position) {
                continue;
            }
            let neighbors = Grid::find_neighbors(position, grid).collect::<Vec<_>>();
            stack.extend(neighbors.iter().map(|(_, np)| *np));
            h.insert(position, neighbors);
        }

        // Convert Position to index values.
        let keys = h.keys.clone();
        let values = h
            .values
            .iter()
            .map(|v| {
                v.iter()
                    .map(|(cost, np)| (*cost, keys.iter().position(|k| k == np).unwrap()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        VecMap { keys, values }
    }

    fn find_neighbors(
        position: Position,
        grid: &Grid,
    ) -> impl Iterator<Item = (usize, Position)> + '_ {
        Grid::neighbors(position, grid).map(move |start_np| {
            let res = (0..).try_fold((1, position, start_np), |(cost, from, to), _| {
                match Grid::single_neighbor(to, from, grid) {
                    Some(next_pos) => ControlFlow::Continue((cost + 1, to, next_pos)),
                    None => ControlFlow::Break((cost, to)),
                }
            });
            match res {
                ControlFlow::Break(x) => x,
                _ => unreachable!(),
            }
        })
    }

    fn single_neighbor(position: Position, from: Position, grid: &Grid) -> Option<Position> {
        let mut neighbors = Grid::neighbors(position, grid).filter(|np| *np != from);
        match (neighbors.next(), neighbors.next()) {
            (Some(np), None) => Some(np),
            _ => None,
        }
    }
}

fn part_1(input: &str) -> usize {
    // Parse input
    let grid = Grid::parse(input);
    let start = Position(0, 1);
    let end = Position(grid.0.len() - 1, grid.0[0].len() - 2);

    Grid::walk_grid(grid, start, end)
}

fn part_2(input: &str) -> usize {
    // Parse input
    let grid = Grid::parse(input);
    let neighbors = Grid::precalc_neighbors(Position(0, 1), &grid);
    let start = neighbors.index_of_key(&Position(0, 1)).unwrap();
    let end = neighbors
        .index_of_key(&Position(grid.0.len() - 1, grid.0[0].len() - 2))
        .unwrap();

    let mut seen = vec![false; neighbors.keys.len()];
    Grid::dfs(start, end, &mut seen, &neighbors.values).unwrap()
}

fn main() -> Result<(), String> {
    let input = fs::read_to_string("in.dat").expect("Could not find file");

    println!("{:-<10} Day 23: A Long Walk {:->10}", "", "");
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

    Ok(())
}
