use std::{fs, time::Instant};

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let seeds = lines.next().unwrap();
    let maps = lines.collect::<Vec<_>>();

    let mut ids: Vec<usize> = seeds
        .split_ascii_whitespace()
        .filter_map(|id| id.parse::<usize>().ok())
        .collect::<Vec<_>>();

    maps.split(|l| l.is_empty())
        .filter(|m| !m.is_empty())
        .for_each(|map| {
            let mut solved = vec![false; ids.len()];
            map.iter().skip(1).for_each(|l| {
                let mapper = l
                    .split(' ')
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                ids.iter_mut().enumerate().for_each(|(i, e)| {
                    let within_range = &mapper[1] <= e && e <= &mut (mapper[1] + mapper[2]);

                    if within_range && !solved[i] {
                        *e = mapper[0] + (*e - mapper[1]);
                        solved[i] = true;
                    }
                });
            });
        });

    *ids.iter().min().unwrap()
}

fn part_2(input: &str) -> usize {
    let (seeds, maps) = input.split_once("\r\n\r\n").unwrap();

    let mut ids = seeds
        .split_ascii_whitespace()
        .filter_map(|id| id.parse::<usize>().ok())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|e| e[0]..(e[0] + e[1]))
        .collect::<Vec<_>>();

    let maps = maps
        .split("\r\n\r\n")
        .map(|m| {
            m.lines()
                .skip(1)
                .map(|l| {
                    l.split_ascii_whitespace()
                        .filter_map(|num| num.parse::<usize>().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for mut map in maps {
        map.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut i = 0;
        loop {
            if i >= ids.len() {
                break;
            }
            let current_range = ids[i].clone();

            for m in map.iter() {
                let destination = m[0];
                let source = m[1];
                let length = m[2];

                let range = source..(source + length);

                let current_start = current_range.start;
                let current_end = current_range.end - 1;

                let start_distance = current_start.saturating_sub(source);
                let end_distance = current_end.saturating_sub(source);

                match (range.contains(&current_start), range.contains(&current_end)) {
                    (true, true) => {
                        ids[i] = (destination + start_distance)..(destination + end_distance);
                        break;
                    }
                    (true, false) => {
                        ids[i] = (destination + start_distance)..(destination + length);
                        let next_range = (source + length)..current_end + 1;
                        ids.insert(i + 1, next_range);
                        break;
                    }
                    (false, true) => {
                        ids[i] = (destination)..(destination + end_distance);
                        let next_range = (current_start)..(source);
                        ids.insert(i + 1, next_range);
                        break;
                    }
                    (false, false) => (),
                }
            }
            i += 1;
        }
    }

    ids.iter().map(|range| range.start).min().unwrap()
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file.");

    println!("--- Day 5: If You Give A Seed A Fertilizer ---");
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
