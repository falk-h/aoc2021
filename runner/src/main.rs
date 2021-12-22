use std::{env, process, time::Instant};

use aoc2021::{Solution, DAY_PART_SEPARATOR, SOLUTIONS};
use humantime::format_duration;

fn list_solutions() -> String {
    SOLUTIONS
        .iter()
        .map(|s| format!("  {}", s))
        .collect::<Vec<_>>()
        .join("\n")
}

fn run_solution(solution: &Solution) {
    print!("Running {}...", solution);

    let input = (solution.parse)();
    let (answer, time) = (solution.run)(input);

    println!(" Done in {}, answer: {}", format_duration(time), answer);
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
        let start = Instant::now();
        SOLUTIONS.iter().for_each(run_solution);
        println!("\nFinished in {}", format_duration(start.elapsed()));
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
                let start = Instant::now();
                solutions.into_iter().for_each(run_solution);
                if num_solutions > 1 {
                    println!("\nFinished in {}", format_duration(start.elapsed()));
                }
            }
        }
    }
}
