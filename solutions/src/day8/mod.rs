use std::collections::HashSet;

pub fn parse(input: Vec<&str>) -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    input
        .into_iter()
        .map(|l| {
            let (l, r) = l.split_once(" | ").unwrap();
            (
                l.split(' ')
                    .map(|s| HashSet::from_iter(s.chars()))
                    .collect::<Vec<_>>(),
                r.split(' ')
                    .map(|s| HashSet::from_iter(s.chars()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>) -> usize {
    let outputs = itertools::concat(input.into_iter().map(|(_, r)| r));
    outputs
        .into_iter()
        .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count()
}

fn decode_mapping(input: &mut Vec<HashSet<char>>) -> Vec<(&HashSet<char>, usize)> {
    assert_eq!(input.len(), 10);
    input.sort_unstable_by_key(|s| s.len());

    // These are easy
    let one = &input[0];
    let four = &input[2];
    let seven = &input[1];
    let eight = &input[9];

    // Nine contains six elements, and all elements of four (unlike zero and six)
    let mut nine_iter = input.iter().filter(|&n| n.len() == 6 && (n | four) == *n);
    let nine = nine_iter.next().unwrap();
    assert_eq!(nine_iter.next(), None);

    // Six contains six elements, is not nine, and contains only one element from one
    let mut six_iter = input
        .iter()
        .filter(|&n| n.len() == 6 && n != nine && (n - one).len() == 5);
    let six = six_iter.next().unwrap();
    assert_eq!(six_iter.next(), None);

    // Zero contains six elements and is neither nine nor six
    let mut zero_iter = input
        .iter()
        .filter(|&n| n.len() == 6 && n != nine && n != six);
    let zero = zero_iter.next().unwrap();
    assert_eq!(zero_iter.next(), None);

    // Three contains five elements and contains all elements from one (unlike two and five)
    let mut three_iter = input.iter().filter(|&n| n.len() == 5 && (n | one) == *n);
    let three = three_iter.next().unwrap();
    assert_eq!(three_iter.next(), None);

    // Five contains five elements, is not three, and contains three elements from four (unlike
    // two, which contains only two)
    let mut five_iter = input
        .iter()
        .filter(|&n| n.len() == 5 && n != three && (n & four).len() == 3);
    let five = five_iter.next().unwrap();
    assert_eq!(five_iter.next(), None);

    // Two contains five elements and shares three elements with five (unlike three)
    let mut two_iter = input
        .iter()
        .filter(|&n| n.len() == 5 && (n & five).len() == 3);
    let two = two_iter.next().unwrap();
    assert_eq!(two_iter.next(), None);

    Vec::from([
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ])
}
fn decode(number: Vec<HashSet<char>>, mapping: Vec<(&HashSet<char>, usize)>) -> usize {
    let mut ret = 0;
    for digit in number {
        ret *= 10;
        let mut num_iter = mapping.iter().filter(|(d, _)| **d == digit);
        let num = num_iter.next().unwrap().1;
        assert_eq!(num_iter.next(), None);
        ret += num;
    }
    ret
}

pub fn part2(input: Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>) -> usize {
    input
        .into_iter()
        .map(|(mut l, r)| decode(r, decode_mapping(&mut l)))
        .sum()
}
