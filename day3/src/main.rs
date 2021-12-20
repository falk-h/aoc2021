use util::*;

make_tests!(part1: 198, part2: 230);

fn part1(input: Vec<&'static str>) -> usize {
    let (epsilon, gamma) = epsilon_gamma(&input);
    epsilon * gamma
}

fn epsilon_gamma(input: &Vec<&'static str>) -> (usize, usize) {
    let len = input.len();
    let mut counts = vec![0; input[0].len()];

    for s in input {
        for (i, c) in s.char_indices() {
            if c == '1' {
                counts[i] += 1
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for c in counts.into_iter() {
        epsilon <<= 1;
        gamma <<= 1;
        if c > len / 2 {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    (epsilon, gamma)
}

fn most_common_bit(input: &Vec<&'static str>, bit_idx: usize) -> bool {
    input
        .into_iter()
        .filter(|&&s| s.as_bytes()[bit_idx] == '1' as u8)
        .count()
        >= (input.len() + 1) / 2
}

fn do_rating_thing(input: &mut Vec<&'static str>, invert: bool) -> usize {
    let bits = input[0].len();
    for i in 0..bits {
        let chr = if most_common_bit(&input, i) ^ invert {
            '1'
        } else {
            '0'
        };
        input.retain(|&n| n.as_bytes()[i] == chr as u8);
        if input.len() == 1 {
            input[0];
            return usize::from_str_radix(input[0], 2).unwrap();
        }
    }
    panic!("wth")
}

fn part2(mut input: Vec<&'static str>) -> usize {
    let o2_rating = do_rating_thing(&mut input.clone(), false);
    let co2_rating = do_rating_thing(&mut input, true);

    o2_rating * co2_rating
}
