use std::fs;

fn main() {
    let input = fs::read_to_string("in.dat").expect("File not found or could not be read.");
    let parsed = parse_input(&input);
    println!("{}", solve(&parsed, false));
    println!("{}", solve(&parsed, true));
}

#[derive(Debug)]
struct Problem {
    stacks: Vec<Vec<char>>,
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Step {
    num_to_move: usize,
    source: usize,
    target: usize,
}

fn parse_input(input: &str) -> Problem {
    let (stacks_str, steps_str) = input.split_once("\r\n\r\n").unwrap();

    let stack_lines = stacks_str.lines().collect::<Vec<&str>>();

    // last line of stacks section is the stack numbers
    let num_stacks = stack_lines[stack_lines.len() - 1]
        .split_ascii_whitespace()
        .count();

    let mut stacks = vec![vec![]; num_stacks];

    for line in stack_lines[0..stack_lines.len() - 1].iter() {
        for (stack_num, crate_char) in line.chars().skip(1).step_by(4).enumerate() {
            if crate_char != ' ' {
                stacks[stack_num].push(crate_char);
            }
        }
    }

    //allows use of push and pop
    for stack in stacks.iter_mut() {
        stack.reverse()
    }

    let steps = steps_str
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();

            Step {
                num_to_move: parts[1].parse::<usize>().unwrap(),
                source: parts[3].parse::<usize>().unwrap() - 1,
                target: parts[5].parse::<usize>().unwrap() - 1,
            }
        })
        .collect::<Vec<Step>>();

    Problem { stacks, steps }
}

fn solve(problem: &Problem, at_once: bool) -> String {
    let mut stacks = problem.stacks.clone();

    for step in problem.steps.iter() {
        let source = &mut stacks[step.source];
        let mut crates_to_move = source.split_off(source.len() - step.num_to_move);

        if !at_once {
            crates_to_move.reverse();
        }

        stacks[step.target].extend_from_slice(&crates_to_move);
    }

    stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<String>()
}
