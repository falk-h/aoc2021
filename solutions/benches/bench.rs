use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    for solution in aoc2021::SOLUTIONS {
        c.bench_function(
            &format!(
                "{}{}{}",
                solution.day,
                aoc2021::DAY_PART_SEPARATOR,
                solution.part
            ),
            |b| {
                b.iter_custom(|iterations| {
                    let mut total = Duration::ZERO;
                    for _ in 0..iterations {
                        let input = (solution.parse)();
                        let (result, time) = (solution.run)(black_box(input));
                        black_box(result); // Make sure rustc doesn't optimize out the calculation of the result
                        total += time;
                    }
                    total
                })
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
