mod util;

use std::{env, fmt::Display, fs, path::Path, process};

use util::Timer;

struct Solution {
    day: &'static str,
    part: &'static str,
    function: &'static dyn Fn(&[&str]) -> usize,
}

const DAY_PART_SEPARATOR: &str = "::";

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.day, DAY_PART_SEPARATOR, self.part)
    }
}

macro_rules! count {
    () => (0usize);
    ($x:tt $($xs:tt)*) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! input {
    ($day:ident) => {{
        // Read different input depending on if we're building tests or not
        #[cfg(not(test))]
        let ret = std::include_str!(std::concat!(std::stringify!($day), "/input.txt"));
        #[cfg(test)]
        let ret = std::include_str!(std::concat!(std::stringify!($day), "/input.test.txt"));
        ret
    }};
}

macro_rules! days {
    {$($day:ident: { $($part:ident: $solution:literal,)+ },)+} => {
        $(
            mod $day;
        )+

        const SOLUTIONS: [Solution; count!($($($part)+)+)] = [
            $(
                $(
                    Solution {
                        day: std::stringify!($day),
                        part: std::stringify!($part),
                        function: &$day::$part,
                    },
                )+
            )+
        ];

        #[cfg(test)]
        mod test {
            $(
                mod $day {
                    $(
                        #[test]
                        fn $part() {
                            std::assert_eq!(
                                crate::$day::$part(&crate::input!($day).split_terminator('\n').collect::<Vec<_>>()),
                                $solution
                            );
                        }
                    )+
                }
            )+
        }
    };
}

days! {
    day1: {
        part1: 7,
        part2: 5,
    },
    day2: {
        part1: 150,
        part2: 900,
    },
    day3: {
        part1: 198,
        part2: 230,
    },
}

fn read_input_lines<P: AsRef<Path>>(file: P) -> Vec<String> {
    fs::read_to_string(file.as_ref())
        .map_err(|e| {
            eprintln!(
                "Failed to read input file {}: {}",
                file.as_ref().to_string_lossy(),
                e
            );
            process::exit(1);
        })
        .unwrap()
        .split_terminator('\n')
        .map(String::from)
        .collect()
}

fn list_solutions() -> String {
    SOLUTIONS
        .iter()
        .map(|s| format!("  {}", s))
        .collect::<Vec<_>>()
        .join("\n")
}

fn run_solution(solution: &Solution) {
    let input = read_input_lines(format!("src/{}/input.txt", solution.day));
    let str_refs = input.iter().map(String::as_str).collect::<Vec<_>>();

    print!("Running {}...", solution);

    let start = Timer::start();
    let answer = (solution.function)(&str_refs);
    let end = start.stop();

    println!(" Done in {}, answer: {}", end, answer);
}

fn usage() {
    let usage = "\
Advent of Code 2021

Specify solutions as commmand line arguments or pass --all to run all of them.
Solutions can be specified as dayX to run the solutions for both parts of that
day, or as dayX::partY to run a specific part.

Pass -h or --help to show this help text.

Available solutions:";

    eprintln!("{}\n{}", usage, list_solutions());
}

fn main() {
    if env::args().count() <= 1 {
        usage();
        process::exit(1);
    }

    let first_arg = env::args().nth(1).unwrap();
    if first_arg == "-h" || first_arg == "--help" {
        usage();
        process::exit(0);
    }

    if env::args().count() == 2 && env::args().nth(1).unwrap() == "--all" {
        SOLUTIONS.iter().for_each(run_solution);
    } else {
        for arg in env::args().skip(1) {
            if let Some((day, part)) = arg.split_once(DAY_PART_SEPARATOR) {
                SOLUTIONS
                    .iter()
                    .filter(|s| s.day == day && s.part == part)
                    .for_each(run_solution)
            } else {
                let solutions = SOLUTIONS
                    .iter()
                    .filter(|s| s.day == arg)
                    .collect::<Vec<_>>();

                if solutions.is_empty() {
                    eprintln!("No such solution: {}", arg);
                    process::exit(1)
                }

                solutions.into_iter().for_each(run_solution)
            }
        }
    }
}
