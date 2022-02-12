#[macro_use]
extern crate criterion;
use criterion::Criterion;

const N: usize = 1_000_000;
const STEP: usize = 101;

fn is_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_prime");

    group.bench_function("is_prime", |b| {
        b.iter(|| {
            (1..N)
                .step_by(STEP)
                .filter(|&n| primal::is_prime(n as u64))
                .count()
        })
    });

    group.bench_function("Sieve::is_prime", |b| {
        let sieve = primal::Sieve::new(N);
        b.iter(|| (1..N).step_by(STEP).filter(|&n| sieve.is_prime(n)).count())
    });

    group.bench_function("Sieve::is_prime with init", |b| {
        b.iter(|| {
            let sieve = primal::Sieve::new(N);
            (1..N).step_by(STEP).filter(|&n| sieve.is_prime(n)).count()
        })
    });

    group.finish();
}

criterion_group!(benches, is_prime);
criterion_main!(benches);
