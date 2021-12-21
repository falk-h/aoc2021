pub fn parse(input: Vec<&str>) -> Vec<&str> {
    input
}

pub fn part1(input: Vec<&str>) -> usize {
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

    epsilon * gamma
}

fn most_common_bit(input: &[&str], bit_idx: usize) -> bool {
    input
        .iter()
        .filter(|&&s| s.as_bytes()[bit_idx] == b'1')
        .count()
        >= (input.len() + 1) / 2
}

fn do_rating_thing(input: &mut Vec<&str>, invert: bool) -> usize {
    let bits = input[0].len();
    for i in 0..bits {
        let chr = if most_common_bit(input, i) ^ invert {
            b'1'
        } else {
            b'0'
        };
        input.retain(|&n| n.as_bytes()[i] == chr as u8);
        if input.len() == 1 {
            return usize::from_str_radix(input[0], 2).unwrap();
        }
    }
    panic!("wth")
}

pub fn part2(mut input: Vec<&str>) -> usize {
    let o2_rating = do_rating_thing(&mut input.clone(), false);
    let co2_rating = do_rating_thing(&mut input, true);

    o2_rating * co2_rating
}
