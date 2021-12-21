mod util;

use std::{env, fmt::Display, process};

use util::Timer;

struct Solution {
    day: &'static str,
    part: &'static str,
    run: &'static dyn Fn() -> usize,
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

#[macro_export]
macro_rules! parsed_input {
    ($day:ident, $parser:ident) => {
        crate::$day::$parser(crate::input!($day).split_terminator('\n').collect::<Vec<_>>())
    };
}

macro_rules! days {
    {$($day:ident: { $parser:ident -> $type:ty, $($part:ident: $solution:literal,)+ },)+} => {
        $(
            mod $day;
        )+

        const SOLUTIONS: [Solution; count!($($($part)+)+)] = [
            $(
                $(
                    Solution {
                        day: std::stringify!($day),
                        part: std::stringify!($part),
                        run: &|| {
                            crate::$day::$part(crate::parsed_input!($day, $parser))
                        },
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
                                crate::$day::$part(crate::parsed_input!($day, $parser)),
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
        parse -> Vec<usize>,
        part1: 7,
        part2: 5,
    },
    day2: {
        parse -> Vec<&str>,
        part1: 150,
        part2: 900,
    },
    day3: {
        parse -> Vec<&str>,
        part1: 198,
        part2: 230,
    },
    day4: {
        parse -> (Vec<usize>, Vec<Bingo>),
        part1: 4512,
        part2: 1924,
    },
    day5: {
        parse -> Vec<Line>,
        part1: 5,
        part2: 12,
    },
    day6: {
        parse -> Vec<usize>,
        part1: 5934,
        part2: 26984457539,
    },
    day7: {
        parse -> Vec<usize>,
        part1: 37,
        part2: 168,
    },
}

fn list_solutions() -> String {
    SOLUTIONS
        .iter()
        .map(|s| format!("  {}", s))
        .collect::<Vec<_>>()
        .join("\n")
}

fn run_solution(solution: &Solution) {
    print!("Running {}...", solution);

    let start = Timer::start();
    let answer = (solution.run)();
    let end = start.stop();

    println!(" Done in {}, answer: {}", end, answer);
}

fn usage() {
    let usage = "\
Advent of Code 2021

Specify solutions as commmand line arguments or pass --all to run all of them.
Solutions can be specified as dayX to run the solutions for both parts of that
day, or as dayX::partY to run a specific part.

Times shown here include the time to parse the input.

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
        let start = Timer::start();
        SOLUTIONS.iter().for_each(run_solution);
        println!("\nFinished in {}", start.stop());
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

                let num_solutions = solutions.len();
                let start = Timer::start();
                solutions.into_iter().for_each(run_solution);
                if num_solutions > 1 {
                    println!("\nFinished in {}", start.stop());
                }
            }
        }
    }
}
