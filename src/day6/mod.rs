struct Fish([usize; 9]);

impl Fish {
    pub fn new(fish: &[usize]) -> Self {
        let mut ret = Self([0; 9]);
        for f in fish {
            ret.0[*f] += 1;
        }
        ret
    }

    pub fn tick(&mut self) {
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
    }

    pub fn count(&self) -> usize {
        self.0.iter().sum()
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

pub fn part1(input: &[&str]) -> usize {
    let mut fish = Fish::new(&parse(input[0]));
    for _ in 0..80 {
        fish.tick();
    }
    fish.count()
}

pub fn part2(input: &[&str]) -> usize {
    let mut fish = Fish::new(&parse(input[0]));
    for _ in 0..256 {
        fish.tick();
    }
    fish.count()
}
