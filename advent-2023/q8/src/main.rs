use std::{collections::HashMap, fs};

type Map = HashMap<String, (String, String)>;

#[derive(Debug)]
struct Network {
    instructions: String,
    map: Map,
}

fn main() {
    println!("--- Day 8: Haunted Wasteland ---");
    let input = fs::read_to_string("in.dat").expect("Could not find file");
    let network: Network = parse(&input);
    println!("Part 1: {}", pt1(&network));
    println!("Part 2: {}", pt2(&network));
}

fn parse(input: &str) -> Network {
    let (instructions, nodes) = input.split_once("\n").unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for node in nodes.trim().lines() {
        let (node_name, els) = node.split_once(" = ").unwrap();
        let elements = els.replace(" (", "").replace(")", "");
        let (el1, el2) = elements.split_once(", ").unwrap();
        map.insert(node_name.to_string(), (el1.to_string(), el2.to_string()));
    }

    Network {
        instructions: instructions.to_string(),
        map,
    }
}

fn pt1(network: &Network) -> i32 {
    let mut cursor: String = "AAA".to_string();

    let mut steps = 0;
    while cursor != "ZZZ" {
        for instruction in network.instructions.chars() {
            let (left, right) = network.map.get(&cursor).unwrap();
            match instruction {
                'L' => cursor = left.to_owned(),
                'R' => cursor = right.to_owned(),
                _ => (),
            }
            steps += 1;
        }
    }

    steps
}

fn pt2(network: &Network) -> usize {
    let mut cursors: Vec<&String> = network
        .map
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect();

    let instruction_counts: Vec<usize> = cursors
        .iter_mut()
        .map(|cursor| {
            let mut i: usize = 0;
            while !cursor.ends_with("Z") {
                for instruction in network.instructions.chars() {
                    let (left, right) = network.map.get(&cursor.to_owned()).unwrap();
                    match instruction {
                        'L' => *cursor = left,
                        'R' => *cursor = right,
                        _ => (),
                    }
                    i += 1;
                }
            }
            i
        })
        .collect();

    let steps = instruction_counts
        .iter()
        .fold(instruction_counts[0], |acc, instruction| {
            lcm(acc, *instruction)
        });

    steps
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}