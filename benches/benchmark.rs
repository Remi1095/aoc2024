use aoc2024::{solution_runners, Runner};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

// fn fibonacci(n: u64) -> u64 {
//     let mut a = 0;
//     let mut b = 1;

//     match n {
//         0 => b,
//         _ => {
//             for _ in 0..n {
//                 let c = a + b;
//                 a = b;
//                 b = c;
//             }
//             b
//         }
//     }
// }

fn criterion_benchmark(criterion: &mut Criterion) {
    let mut solution_runnners = solution_runners().into_iter().collect::<Vec<_>>();
    solution_runnners.sort_by_key(|(day, _)| *day);

    for (day, runners) in solution_runnners.iter() {
        for (part, runner) in runners.iter().enumerate() {
            criterion.bench_function(&format!("Day {} part {}", day, part+1), |bencher| {
                bencher.iter(runner)
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
