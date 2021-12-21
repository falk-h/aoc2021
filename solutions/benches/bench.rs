use std::{any::Any, fmt::Display, time::{Duration, Instant}};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

struct BenchmarkInput<'a>(&'a Box<dyn Any>);

impl Display for BenchmarkInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<benchmark input>")
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    for solution in aoc2021::SOLUTIONS {
        c.bench_function(
            &format!("{}{}{}", solution.day, aoc2021::DAY_PART_SEPARATOR, solution.part),
            |b| {
                b.iter_custom(|iterations| {
                    let mut total = Duration::ZERO;
                    for _ in 0..iterations {
                        let input = (solution.parse)();
                        let start = Instant::now();
                        (solution.run)(black_box(input));
                        total += start.elapsed();
                    }
                    total
                })
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
