use std::{
    collections::{HashSet, VecDeque},
    fs,
    time::Instant,
};

type BrickID = usize;
const NO_BRICK: BrickID = usize::MAX;

#[derive(Clone)]
struct Brick {
    side1: [usize; 3],
    side2: [usize; 3],
    id: usize,
    below: Vec<BrickID>,
    above: Vec<BrickID>,
}

impl Brick {
    fn new(coords: &[usize]) -> Self {
        Self {
            side1: [coords[0], coords[1], coords[2]],
            side2: [coords[3], coords[4], coords[5]],
            id: 0,
            below: Vec::with_capacity(10),
            above: Vec::with_capacity(10),
        }
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn lo_z(&self) -> usize {
        self.side1[2].min(self.side2[2])
    }

    fn hi_z(&self) -> usize {
        self.side1[2].max(self.side2[2])
    }

    fn can_safely_remove(&self, bricks: &[Brick]) -> bool {
        for &b in &self.above {
            if bricks[b].below.len() == 1 {
                return false;
            }
        }
        true
    }

    fn land(&mut self, platform: &mut [[BrickID; 10]; 10], bricks: &mut [Brick]) {
        let mut below = HashSet::new();
        let mut high = 0;

        for x in self.side1[0]..=self.side2[0] {
            for y in self.side1[1]..=self.side2[1] {
                if platform[x][y] != NO_BRICK {
                    high = high.max(bricks[platform[x][y]].hi_z());
                    below.insert(platform[x][y]);
                }
                platform[x][y] = self.id;
            }
        }

        for id in below {
            if bricks[id].hi_z() == high {
                self.below.push(id);
                bricks[id].above.push(self.id);
            }
        }

        if self.side1[2] < self.side2[2] {
            let d = self.side2[2] - self.side1[2];
            self.side1[2] = high + 1;
            self.side2[2] = self.side1[2] + d;
        } else {
            let d = self.side1[2] - self.side2[2];
            self.side2[2] = high + 1;
            self.side1[2] = self.side2[2] + d;
        }
    }

    fn parse(input: &str) -> Result<Vec<Brick>, String> {
        // Parse input into bricks
        let mut bricks = Vec::new();

        for line in input.lines() {
            let points = line
                .split(&[',', '~'])
                .map(|s| s.parse())
                .collect::<Result<Vec<usize>, _>>()
                .map_err(|_| "Bad input")?;
            bricks.push(Brick::new(&points));
        }

        // Assign ids to each bricks
        bricks.sort_by_key(|b| b.lo_z());
        bricks.iter_mut().enumerate().for_each(|(i, b)| b.set_id(i));

        Ok(bricks)
    }

    fn place_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
        let mut hgrid = [[NO_BRICK; 10]; 10];

        // Place bricks
        for b in 0..bricks.len() {
            let mut brick = bricks[b].clone();
            brick.land(&mut hgrid, &mut bricks);
            bricks[b] = brick;
        }

        bricks
    }

    fn will_fall(&self, is_falling: &[bool]) -> bool {
        self.below.iter().all(|&b| is_falling[b])
    }
}

fn part_1(input: &str) -> Result<usize, String> {
    // Parse input into bricks and place them
    let bricks = Brick::place_bricks(Brick::parse(input)?);

    // Remove safe bricks
    let mut count = 0;
    for b in &bricks {
        if b.can_safely_remove(&bricks) {
            count += 1;
        }
    }

    Ok(count)
}

fn part_2(input: &str) -> Result<usize, String> {
    // Parse input into bricks and place them
    let bricks = Brick::place_bricks(Brick::parse(input)?);

    let mut count = 0;
    let mut is_falling = vec![false; bricks.len()];
    let mut queue = VecDeque::new();

    for b in &bricks {
        is_falling[b.id] = true;
        queue.push_back(b.id);

        while let Some(b) = queue.pop_front() {
            for b in &bricks[b].above {
                if !is_falling[*b] && bricks[*b].will_fall(&is_falling) {
                    is_falling[*b] = true;
                    queue.push_back(*b);
                    count += 1;
                }
            }
        }
        is_falling.fill(false);
    }

    Ok(count)
}

fn main() -> Result<(), String> {
    let input = fs::read_to_string("in.dat").expect("Could not find file");

    println!("{:-<10} Day 22: Sand Slabs {:->10}", "", "");
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
