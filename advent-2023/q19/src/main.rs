use std::{collections::HashMap, fs, time::Instant};

use regex::Regex;

#[derive(Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_str(category: &str) -> Self {
        match category {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => panic!("Bad input to map category."),
        }
    }
    fn to_str(category: &Category) -> String {
        match category {
            Category::X => "x",
            Category::M => "m",
            Category::A => "a",
            Category::S => "s",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
struct Filter {
    gtr: bool,
    op1: Category,
    op2: usize,
    dst: String,
}
impl Filter {
    fn new(gtr: bool, op1: Category, op2: usize, dst: &str) -> Self {
        Filter {
            gtr,
            op1,
            op2,
            dst: dst.to_owned(),
        }
    }
    /// Accpets or Rejects a Part based on the filter conditions
    fn accept(&self, part: &Part) -> Option<&str> {
        // Grabs relevant category of the part
        let category = part.get_category(&self.op1);

        // Checks if part's category fulfills the condition
        let filter_result = if self.gtr {
            category > self.op2
        } else {
            category < self.op2
        };

        // If bool is true wrap destination in `Some()` otherwise return `None`
        filter_result.then_some(&self.dst)
    }

    fn from_str(filter: &str, filter_reg: &Regex) -> Self {
        let filter_capture = filter_reg.captures(filter).unwrap();

        let op1 = filter_capture.get(1).map_or("", |m| m.as_str());
        let gtr = filter_capture.get(2).map_or(false, |m| match m.as_str() {
            "<" => false,
            ">" => true,
            _ => panic!("Found invalid &str instead of < or >"),
        });
        let op2 = filter_capture
            .get(3)
            .map(|m| m.as_str())
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let dst = filter_capture.get(4).map_or("", |m| m.as_str());

        Filter::new(gtr, Category::from_str(op1), op2, dst)
    }

    fn constrain(&self, aff: bool, dom: &mut Vec<Vec<bool>>) {
        let set = &mut dom["xmas".find(&Category::to_str(&self.op1)).unwrap()];
        let r = if aff {
            if self.gtr {
                1..=self.op2
            } else {
                self.op2..=4000
            }
        } else if self.gtr {
            self.op2 + 1..=4000
        } else {
            1..=self.op2 - 1
        };

        for i in r {
            set[i] = false;
        }
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Part { x, m, a, s }
    }

    fn parse(part: &str) -> Self {
        let reg =
            Regex::new(r"\{x=([0-9]{1,}),m=([0-9]{1,}),a=([0-9]{1,}),s=([0-9]{1,})}").unwrap();

        let captures = reg.captures(part).unwrap();

        let x = captures
            .get(1)
            .map(|m| m.as_str().parse().unwrap())
            .unwrap();
        let m = captures
            .get(2)
            .map(|m| m.as_str().parse().unwrap())
            .unwrap();
        let a = captures
            .get(3)
            .map(|m| m.as_str().parse().unwrap())
            .unwrap();
        let s = captures
            .get(4)
            .map(|m| m.as_str().parse().unwrap())
            .unwrap();

        Part::new(x, m, a, s)
    }

    fn get_count(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    /// Takes in a `Category` enum and returns the value of the part's category that the `Category`
    /// enum asks for.
    fn get_category(&self, category: &Category) -> usize {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    dest: String,
    filters: Vec<Filter>,
}

impl Workflow {
    fn new(dest: &str, filters: Vec<Filter>) -> Self {
        Workflow {
            dest: dest.to_owned(),
            filters,
        }
    }

    /// Iterates over all tests and if part fulfills one of them it returns the name of the new
    /// `Workflow` the part should be past to. If it fulfiils no tests then the current
    /// `Workflow`'s name is returned.
    fn eval(&self, part: &Part) -> &str {
        for test in &self.filters {
            if let Some(x) = test.accept(part) {
                return x;
            }
        }

        &self.dest
    }
}

#[derive(Debug)]
struct System {
    workflows: HashMap<String, Workflow>,
}

impl System {
    fn new(workflows: HashMap<String, Workflow>) -> Self {
        System { workflows }
    }

    fn parse(input: &str) -> Self {
        // Hellish regex that grabs name and needed features for building the `Workflow`
        let line_reg =
            Regex::new(r"([a-z]{2,3})\{((?:[xmas][<>][0-9]+:[[a-z][AR]]+,)+)([[a-z][AR]]+)}")
                .unwrap();

        let filter_reg = Regex::new(r"([xmas])([<>])([0-9]+):([[a-z][AR]]+)").unwrap();

        let mut workflows = HashMap::new();

        input.lines().for_each(|line| {
            let line_capture = line_reg.captures(line).unwrap();
            // The name becomes the key into the hashmap
            let name = line_capture.get(1).map_or("", |m| m.as_str());
            // Tests from the workflow
            let filters = line_capture.get(2).map_or("", |m| m.as_str());
            let filters = &filters[..(filters.len() - 1)];
            let filters = filters
                .split(',')
                .map(|filter| Filter::from_str(filter, &filter_reg))
                .collect::<Vec<Filter>>();

            // Destination for the workflow
            let dest = line_capture.get(3).map_or("", |m| m.as_str());

            let workflow = Workflow::new(dest, filters);

            workflows.insert(name.to_owned(), workflow);
        });

        System::new(workflows)
    }

    fn process_part(&self, part: &Part) -> bool {
        let mut key = "in";
        loop {
            let result = self.workflows[key].eval(part);
            match result {
                "A" => break true,
                "R" => break false,
                _ => key = result,
            }
        }
    }

    /// For every accept the number of unique part evaluations that can lead to that accept state
    /// are counted. The total number is then return.
    fn permute_possiblities(&self, key: &str, mut seq: Vec<(Filter, bool)>) -> usize {
        match key {
            "A" => {
                let mut permutation = (0..4)
                    .map(|_| {
                        let mut vec = vec![true; 4001];
                        vec[0] = false;
                        vec
                    })
                    .collect();

                for (filter, is_accepted) in seq {
                    filter.constrain(is_accepted, &mut permutation);
                }

                permutation
                    .iter()
                    .map(|v| v.iter().filter(|f| **f).count())
                    .product()
            }
            "R" => 0,
            _ => {
                let workflow = &self.workflows[key];
                let mut n = 0;

                for filter in &workflow.filters {
                    let mut seq_clone = seq.clone();
                    seq_clone.push((filter.clone(), true));
                    n += self.permute_possiblities(&filter.dst, seq_clone);
                    seq.push((filter.clone(), false));
                }
                n += self.permute_possiblities(&workflow.dest, seq);
                n
            }
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut split = input.split("\n\n");
    let system = System::parse(split.next().unwrap());
    let mut total = 0;

    for line in split.next().unwrap().lines() {
        let part = Part::parse(line);

        let is_accepted = system.process_part(&part);

        if is_accepted {
            total += part.get_count();
        }
    }

    total
}

fn part_2(input: &str) -> usize {
    let mut split = input.split("\n\n");
    let system = System::parse(split.next().unwrap());

    system.permute_possiblities("in", Vec::new())
}

fn main() {
    let input = fs::read_to_string("in.dat").expect("Could not find file");

    println!("{:-<15} Day 19: Aplenty {:->15}", "", "");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_accept_returns_properly() {
        let filter = Filter::new(true, Category::S, 20, "Test");
        let part = Part::new(0, 0, 0, 21);

        assert_eq!(filter.accept(&part), Some("Test"));

        let part = Part::new(0, 0, 0, 19);

        assert_eq!(filter.accept(&part), None);

        let filter = Filter::new(false, Category::S, 20, "Test");

        assert_eq!(filter.accept(&part), Some("Test"));
    }

    #[test]
    fn workflow_eval_returns_properly() {
        let filter1 = Filter::new(true, Category::X, 20, "Test1");
        let filter2 = Filter::new(false, Category::M, 20, "Test2");
        let filter3 = Filter::new(true, Category::A, 20, "Test3");
        let filter4 = Filter::new(false, Category::S, 20, "Test4");

        let part1 = Part::new(21, 0, 0, 0);
        let part2 = Part::new(0, 19, 0, 0);
        let part3 = Part::new(0, 20, 21, 0);
        let part4 = Part::new(0, 20, 0, 19);
        let part5 = Part::new(0, 20, 0, 20);

        let workflow = Workflow::new("Test", vec![filter1, filter2, filter3, filter4]);

        assert_eq!(workflow.eval(&part1), "Test1");
        assert_eq!(workflow.eval(&part2), "Test2");
        assert_eq!(workflow.eval(&part3), "Test3");
        assert_eq!(workflow.eval(&part4), "Test4");
        assert_eq!(workflow.eval(&part5), "Test");
    }
}
