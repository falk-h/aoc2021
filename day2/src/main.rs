use util::*;

make_tests!(part1: 150, part2: 900);

#[derive(Default, Debug)]
struct Submarine {
    horizontal: usize,
    vertical: usize,
    aim: usize,
}

fn part1(input: Vec<&'static str>) -> usize {
    let result = input.into_iter().fold(Submarine::default(), |acc, c| {
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

fn part2(input: Vec<&'static str>) -> usize {
    let result = input.into_iter().fold(Submarine::default(), |acc, c| {
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
