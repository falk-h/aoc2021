use std::{str::FromStr, time::Instant};

use humantime::format_duration;

pub struct Timer(Instant);

impl Timer {
    pub fn start() -> Self {
        Self(Instant::now())
    }

    pub fn stop(self) -> String {
        format_duration(self.0.elapsed()).to_string()
    }
}

#[macro_export]
macro_rules! input {
    () => {{
        #[cfg(not(test))]
        let ret = std::include_str!("../input.txt");
        #[cfg(test)]
        let ret = std::include_str!("../input.test.txt");
        ret
    }};
}

#[macro_export]
macro_rules! make_main {
    ($($fn:ident),+) => {
        fn main() {
            let input: Vec<&'static str> = crate::input!().split_terminator('\n').collect();

            $(
                {
                    let input = input.clone();
                    std::print!("Running {}...", stringify!($fn));
                    let timer = crate::Timer::start();
                    let result = $fn(input);
                    let time = timer.stop();
                    std::println!(" {}", time);
                    std::println!("{}: {}", std::stringify!($fn), result);
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! make_test {
    ($fn_name:ident, $solution:literal) => {
        #[test]
        fn $fn_name() {
            std::assert_eq!(
                super::$fn_name(crate::input!().split_terminator('\n').collect()),
                $solution
            );
        }
    };
}

#[macro_export]
macro_rules! make_tests {
    ($part1_fn:ident: $part1_solution:literal) => {
        crate::make_main!($part1_fn);

        #[cfg(test)]
        mod generated_tests {
            crate::make_test!($part1_fn, $part1_solution);
        }
    };
    ($part1_fn:ident: $part1_solution:literal, $part2_fn:ident: $part2_solution:literal) => {
        crate::make_main!($part1_fn, $part2_fn);

        #[cfg(test)]
        mod generated_tests {
            crate::make_test!($part1_fn, $part1_solution);
            crate::make_test!($part2_fn, $part2_solution);
        }
    };
}

pub fn read_vec<T: FromStr>(input: &str) -> Vec<T> {
    parse_strings(input.split_terminator('\n'))
}

fn parse_strings<'a, T: FromStr, I: Iterator<Item = &'a str>>(strings: I) -> Vec<T> {
    strings.map(|s| T::from_str(s).ok().unwrap()).collect()
}
