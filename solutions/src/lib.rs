mod util;

use std::{any::Any, fmt::Display};

pub struct Solution {
    pub day: &'static str,
    pub part: &'static str,
    pub run: &'static dyn Fn(Box<dyn Any>) -> usize,
    pub parse: &'static dyn Fn() -> Box<dyn Any>,
}

pub const DAY_PART_SEPARATOR: &str = "::";

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

        pub const SOLUTIONS: [Solution; count!($($($part)+)+)] = [
            $(
                $(
                    Solution {
                        day: std::stringify!($day),
                        part: std::stringify!($part),
                        parse: &|| {
                            std::boxed::Box::new(crate::parsed_input!($day, $parser))
                        },
                        run: &|input| {
                            crate::$day::$part(*input.downcast().unwrap())
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
