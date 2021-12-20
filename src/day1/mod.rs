pub fn part1(input: &[&str]) -> usize {
    input
        .into_iter()
        .fold((0, "9999"), move |(count, last), &x| {
            (count + ((x > last) as usize), x)
        })
        .0
}

pub fn part2(input: &[&str]) -> usize {
    input
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|c| c.into_iter().sum::<usize>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|ns| ns[0] < ns[1])
        .count()
}
