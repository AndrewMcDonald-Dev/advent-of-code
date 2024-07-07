use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs,
    time::Instant,
};

use regex::Regex;

#[derive(PartialEq, Clone)]
enum Pulse {
    Low,
    High,
}

type Identifier = String;

#[derive(Clone)]
enum ModuleRole {
    Conjunction { recent: HashMap<Identifier, Pulse> },
    FlipFlop { recent: Pulse, is_on: bool },
    Broadcast { recent: Pulse },
    Button,
}

#[derive(Clone)]
struct Module {
    identifier: Identifier,
    role: ModuleRole,
    receivers: Vec<String>,
    n_high: u32,
    n_low: u32,
}

impl Module {
    fn new(identifier: &str, role: ModuleRole, receivers: Vec<String>) -> Self {
        Self {
            identifier: identifier.into(),
            receivers,
            role,
            n_high: 0,
            n_low: 0,
        }
    }
    fn new_button(identifier: &str, receiver: &str) -> Self {
        Self::new(identifier, ModuleRole::Button, vec![receiver.into()])
    }
    fn new_conjunction(identifier: &str, targets: Vec<Identifier>) -> Self {
        Self::new(
            identifier,
            ModuleRole::Conjunction {
                recent: HashMap::new(),
            },
            targets,
        )
    }
    fn new_flip_flop(identifier: &str, targets: Vec<Identifier>) -> Self {
        Self::new(
            identifier,
            ModuleRole::FlipFlop {
                recent: Pulse::Low,
                is_on: false,
            },
            targets,
        )
    }
    fn new_broadcast(identifier: &str, targets: Vec<Identifier>) -> Self {
        Self::new(
            identifier,
            ModuleRole::Broadcast { recent: Pulse::Low },
            targets,
        )
    }

    fn counts(&self) -> (u32, u32) {
        (self.n_high, self.n_low)
    }

    fn receive(&mut self, source: &str, pulse: Pulse) {
        match self.role {
            ModuleRole::Conjunction { ref mut recent } => {
                if let Some(input) = recent.get_mut(source) {
                    *input = pulse;
                }
            }
            ModuleRole::FlipFlop { ref mut recent, .. } => {
                *recent = pulse;
            }
            ModuleRole::Broadcast { ref mut recent } => {
                *recent = pulse;
            }
            ModuleRole::Button => { /* Not a receiver */ }
        }
    }

    fn send(&mut self, mediator: &mut Mediator) {
        match self.role {
            ModuleRole::Conjunction { ref recent } => {
                let pulse = if recent.values().all(|p| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                self.count_and_target_all(mediator, pulse);
            }
            ModuleRole::FlipFlop {
                ref recent,
                ref mut is_on,
            } => {
                if *recent == Pulse::Low {
                    *is_on = !*is_on;
                    let pulse = if *is_on { Pulse::High } else { Pulse::Low };

                    self.count_and_target_all(mediator, pulse);
                }
            }
            ModuleRole::Broadcast { ref recent } => {
                self.count_and_target_all(mediator, recent.clone());
            }
            ModuleRole::Button => {
                self.count_and_target_all(mediator, Pulse::Low);
            }
        }
    }

    fn count_and_target_all(&mut self, mediator: &mut Mediator, pulse: Pulse) {
        if pulse == Pulse::High {
            self.n_high += 1;
        } else {
            self.n_low += 1;
        }

        for target in &self.receivers {
            mediator.send(&self.identifier, target, pulse.clone());
        }
    }
}

struct Mediator {
    queue: VecDeque<(Identifier, Identifier, Pulse)>,
    n_high: u32,
    n_low: u32,
}

impl Mediator {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            n_high: 0,
            n_low: 0,
        }
    }
    fn get_pulse_counts(&self) -> u32 {
        self.n_high * self.n_low
    }

    fn send(&mut self, source: &str, target: &str, pulse: Pulse) {
        if pulse == Pulse::High {
            self.n_high += 1;
        } else {
            self.n_low += 1;
        }

        self.queue.push_back((source.into(), target.into(), pulse));
    }
    fn loop_until_done(&mut self, modules: &mut HashMap<Identifier, Module>) {
        while let Some((source, target, pulse)) = self.queue.pop_front() {
            if let Some(module) = modules.get_mut(&target) {
                module.receive(&source, pulse);
                module.send(self);
            }
        }
    }
}

fn parse_input(input: &str) -> Result<HashMap<String, Module>, Box<dyn Error>> {
    let line_reg = Regex::new(r"([%&]?)(\w+) +-> +(.*)")?;
    let targets_reg = Regex::new(r"(\w+)")?;
    let mut modules = HashMap::<String, Module>::new();

    for line in input.lines() {
        let caps = line_reg.captures(line).ok_or("No Match")?;
        let (_, [pfx, name, targets]) = caps.extract();
        let targets = targets_reg
            .captures_iter(targets)
            .map(|c| c[1].into())
            .collect::<Vec<String>>();

        let module = {
            match pfx {
                "&" => Module::new_conjunction(name, targets),
                "%" => Module::new_flip_flop(name, targets),
                "" => Module::new_broadcast(name, targets),
                _ => return Err("Invalid prefix".into()),
            }
        };
        modules.insert(name.into(), module);
    }

    for (sender_id, module) in modules.clone().iter() {
        for receiver_id in &module.receivers {
            if let Some(receiver) = modules.get_mut(receiver_id) {
                if let ModuleRole::Conjunction { ref mut recent } = receiver.role {
                    recent.insert(sender_id.into(), Pulse::Low);
                }
            }
        }
    }
    Ok(modules)
}

fn part_1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut modules = parse_input(input)?;
    let mut mediator = Mediator::new();
    let mut button = Module::new_button("button", "broadcaster");

    for _ in 0..1000 {
        button.send(&mut mediator);
        mediator.loop_until_done(&mut modules);
    }

    Ok(mediator.get_pulse_counts())
}

fn part_2(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut modules = parse_input(input)?;
    let mut mediator = Mediator::new();
    let mut button = Module::new_button("button", "broadcaster");

    let df_inputs = ["xl", "ln", "xp", "gp"];
    let mut df_counts = [0; 4];

    for press_n in 1..10000 {
        button.send(&mut mediator);
        mediator.loop_until_done(&mut modules);

        for (df_in, df_c) in df_inputs.iter().cloned().zip(df_counts.iter_mut()) {
            if let Some(module) = modules.get(df_in) {
                if module.counts().0 > 0 && df_c == &0 {
                    *df_c = press_n;
                }
            }
        }
        if df_counts.iter().all(|d| *d > 0) {
            break;
        }
    }

    Ok(df_counts.iter().product::<u32>())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("in.dat").expect("Could not find file");

    println!("{:-<10} Day 20: Pulse Propagation {:->10}", "", "");
    let before = Instant::now();
    println!(
        "Part 1: {:<15} | Elapsed Time: {:.2?}",
        part_1(&input)?,
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "Part 2: {:<15} | Elapsed Time: {:.2?}",
        part_2(&input)?,
        before.elapsed()
    );

    Ok(())
}
