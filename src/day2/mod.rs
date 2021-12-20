#[derive(Default, Debug)]
struct Submarine {
    horizontal: usize,
    vertical: usize,
    aim: usize,
}

pub fn part1(input: &[&str]) -> usize {
    let result = input.iter().fold(Submarine::default(), |acc, c| {
        match c.split_once(' ').unwrap() {
            ("forward", n) => Submarine {
                horizontal: acc.horizontal + n.parse::<usize>().unwrap(),
                ..acc
            },
            ("down", n) => Submarine {
                vertical: acc.vertical + n.parse::<usize>().unwrap(),
                ..acc
            },
            ("up", n) => Submarine {
                vertical: acc.vertical - n.parse::<usize>().unwrap(),
                ..acc
            },
            x => panic!("Got {:?}!", x),
        }
    });
    result.horizontal * result.vertical
}

pub fn part2(input: &[&str]) -> usize {
    let result = input.iter().fold(Submarine::default(), |acc, c| {
        match c.split_once(' ').unwrap() {
            ("forward", n) => Submarine {
                horizontal: acc.horizontal + n.parse::<usize>().unwrap(),
                vertical: acc.vertical + n.parse::<usize>().unwrap() * acc.aim,
                ..acc
            },
            ("down", n) => Submarine {
                aim: acc.aim + n.parse::<usize>().unwrap(),
                ..acc
            },
            ("up", n) => Submarine {
                aim: acc.aim - n.parse::<usize>().unwrap(),
                ..acc
            },
            x => panic!("Got {:?}!", x),
        }
    });
    result.horizontal * result.vertical
}
