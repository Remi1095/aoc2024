use aoc2024::solution_runners;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(criterion: &mut Criterion) {
    let mut solution_runnners = solution_runners().into_iter().collect::<Vec<_>>();
    solution_runnners.sort_by_key(|(day, _)| *day);

    for (day, runners) in solution_runnners.iter() {
        for (part, runner) in runners.iter().enumerate() {
            criterion.bench_function(&format!("day {} part {}", day, part + 1), |bencher| {
                bencher.iter(runner)
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
