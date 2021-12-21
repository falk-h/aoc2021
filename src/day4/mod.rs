use std::{convert::identity, fmt::Display};

use crate::util;

#[derive(Debug)]
struct Bingo {
    rows: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
}

impl Display for Bingo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for (i, row) in self.rows.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                let n = format!("{:>2}", n);
                if self.marked[i][j] {
                    ret.push_str("\x1b[32m")
                }
                ret.push_str(&format!("{:>2} ", n));
                if self.marked[i][j] {
                    ret.push_str("\x1b[m\x0f")
                }
            }
            ret.push('\n');
        }

        write!(f, "{}", ret)
    }
}

impl Bingo {
    pub fn parse(s: &str) -> Self {
        let rows = s
            .split_terminator('\n')
            .map(|s| {
                s.split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), rows[0].len()); // Make sure our bingo is square

        let marked = util::matrix(false, rows.len(), rows[0].len());

        Self { rows, marked }
    }

    fn calculate_score(&self, num: usize) -> usize {
        let mut sum = 0;
        for (i, row) in self.marked.iter().enumerate() {
            for (j, is_marked) in row.iter().enumerate() {
                if !*is_marked {
                    sum += self.rows[i][j];
                }
            }
        }
        sum * num
    }

    fn check_win(&self, num: usize) -> Option<usize> {
        if self
            .marked
            .iter()
            .map(|row| row.iter().all(|b| *b))
            .any(identity)
        {
            Some(self.calculate_score(num))
        } else {
            for j in 0..self.marked[0].len() {
                if self.marked.iter().map(|row| row[j]).all(identity) {
                    return Some(self.calculate_score(num));
                }
            }
            None
        }
    }

    pub fn mark(&mut self, num: usize) -> Option<usize> {
        if let Some((i, j)) = self.find(num) {
            assert!(!self.marked[i][j]); // I don't think the same number should show up twice
            self.marked[i][j] = true;
            self.check_win(num)
        } else {
            None
        }
    }

    pub fn find(&self, needle: usize) -> Option<(usize, usize)> {
        for (i, row) in self.rows.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                if *n == needle {
                    assert_eq!(self.rows[i][j], *n); // Make sure I haven't flipped around i and j
                    return Some((i, j));
                }
            }
        }

        None
    }
}

fn parse(input: &[&str]) -> (Vec<usize>, Vec<Bingo>) {
    let nums = input[0]
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(input[1], ""); // The second line should be empty

    let bingos = input[2..]
        .join("\n")
        .split("\n\n")
        .map(Bingo::parse)
        .collect::<Vec<_>>();

    (nums, bingos)
}

pub fn part1(input: &[&str]) -> usize {
    let (nums, mut bingos) = parse(input);

    for n in nums {
        for bingo in &mut bingos {
            if let Some(score) = bingo.mark(n) {
                return score;
            }
        }
    }

    panic!("No winner!")
}

pub fn part2(input: &[&str]) -> usize {
    let (nums, mut bingos) = parse(input);

    for n in nums {
        let mut to_remove: Vec<usize> = Vec::new();
        let only_one_left = bingos.len() == 1;
        for (i, bingo) in bingos.iter_mut().enumerate() {
            if let Some(score) = bingo.mark(n) {
                if only_one_left {
                    return score;
                } else {
                    to_remove.push(i)
                }
            }
        }
        to_remove.reverse(); // Remove elements from the back so indexes don't shift around
        for i in to_remove {
            bingos.remove(i);
        }
    }

    panic!("No winner!")
}
