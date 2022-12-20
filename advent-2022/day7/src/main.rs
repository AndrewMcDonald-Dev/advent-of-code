use regex::Regex;
use std::fs;
fn main() {
    let input = fs::read_to_string("in.dat").expect("File not found or could not be read.");
    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}

#[allow(dead_code)]
#[derive(Debug)]
struct Directory {
    name: String,
    size: usize,
    parent: usize,
}

fn parse(input: &str) -> Vec<Directory> {
    //parent is the index in the vec of the parent Directory
    //size is the accumulative size of all files and directories beneath

    let mut directories = vec![Directory {
        name: "/".to_string(),
        size: 0,
        parent: 0,
    }];

    let root = 0;
    let mut cwd = root;

    //only care about the changing of the dir
    let dir_rx = Regex::new(r"\s*\$ cd (?P<target>.+)").unwrap();
    //and files for their size
    let file_rx = Regex::new(r"\s*(?P<size>\d+)\s+(?P<file>.+)").unwrap();

    for line in input.lines() {
        if let Some(cap) = dir_rx.captures(line) {
            match &cap["target"] {
                "/" => cwd = root,
                ".." => cwd = directories[cwd].parent,
                name => {
                    directories.push(Directory {
                        name: String::from(name),
                        size: 0,
                        parent: cwd,
                    });
                    cwd = directories.len() - 1;
                }
            }
        } else if let Some(cap) = file_rx.captures(line) {
            //could store files but no need
            let size = &cap["size"].parse().unwrap();

            let mut p = cwd;
            loop {
                directories[p].size += size;
                if p == root {
                    break;
                }
                p = directories[p].parent;
            }
        }
    }
    directories
}

fn part_a(input: &str) -> usize {
    let file_sys = parse(input);

    file_sys
        .iter()
        .map(|d| d.size)
        .filter(|size| *size <= 100000)
        .sum()
}

fn part_b(input: &str) -> usize {
    let file_sys = parse(input);
    let free_space = 70000000 - file_sys[0].size;
    let need_to_free = 30000000 - free_space;

    file_sys
        .iter()
        .map(|d| d.size)
        .filter(|size| *size >= need_to_free)
        .min()
        .expect("at lease one dir must be deleted.")
}
