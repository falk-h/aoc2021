use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::util;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<isize>>,
}

impl Point {
    pub fn parse(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    pub fn signum(&self) -> Self {
        Point {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Line {
    pub fn parse(s: &str) -> Self {
        let (start, end) = s.split_once(" -> ").unwrap();
        Self {
            start: Point::parse(start),
            end: Point::parse(end),
        }
    }

    pub fn straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    pub fn direction(&self) -> Point {
        (self.end - self.start).signum()
    }
}

impl Board {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            rows: util::matrix(0isize, y_size, x_size),
        }
    }

    pub fn add_line(&mut self, line: &Line) {
        let mut pos = line.start;
        let step = line.direction();
        pos -= step;
        while pos != line.end {
            pos += step;
            self.rows[pos.y as usize][pos.x as usize] += 1;
        }
    }

    pub fn num_overlaps(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.iter().filter(|&&n| n > 1).count())
            .sum()
    }
}

pub fn parse(input: Vec<&str>) -> Vec<Line> {
    input.iter().map(|s| Line::parse(s)).collect::<Vec<_>>()
}

fn board_size(lines: &[Line]) -> (usize, usize) {
    lines.iter().fold((0, 0), |(x, y), l| {
        (
            x.max(l.start.x as usize).max(l.end.x as usize),
            y.max(l.start.y as usize).max(l.end.y as usize),
        )
    })
}

pub fn part1(input: Vec<Line>) -> usize {
    let lines = input
        .into_iter()
        .filter(Line::straight)
        .collect::<Vec<_>>();
    let (x, y) = board_size(&lines);
    let mut board = Board::new(x + 1, y + 1);

    for line in lines {
        board.add_line(&line);
    }

    board.num_overlaps()
}

pub fn part2(input: Vec<Line>) -> usize {
    let (x, y) = board_size(&input);
    let mut board = Board::new(x + 1, y + 1);

    for line in input {
        board.add_line(&line);
    }

    board.num_overlaps()
}
