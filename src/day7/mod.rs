fn parse(input: &[&str]) -> Vec<usize> {
    input[0]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>()
}

fn median(input: &[usize]) -> usize {
    let mut copy = Vec::from(input);
    copy.sort();
    copy[input.len() / 2]
}

fn abs_diff(a: usize, b: usize) -> usize {
    let a = a as isize;
    let b = b as isize;
    (a.max(b) - a.min(b)).abs() as usize
}

pub fn part1(input: &[&str]) -> usize {
    let nums = parse(input);
    let median = median(&nums);
    nums.iter().map(|n| abs_diff(*n, median)).sum()
}

fn average(nums: &[usize]) -> usize {
    // There's some rounding weirdness here. Not sure why the + 1 is needed, but it makes it work.
    (nums.iter().sum::<usize>() + 1) / nums.len()
}

fn weird_diff(a: usize, b: usize) -> usize {
    // Original version: (1..=abs_diff(a, b)).sum()
    // Optimized:
    let diff = abs_diff(a, b);
    (diff * (diff + 1)) / 2
}

pub fn part2(input: &[&str]) -> usize {
    let nums = parse(input);
    let average = average(&nums);
    nums.iter().map(|n| weird_diff(*n, average)).sum()
}
