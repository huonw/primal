#[macro_use]
extern crate criterion;
extern crate primal;
use criterion::{Criterion, Fun};

const N: usize = 1_000_000;
const STEP: usize = 101;

fn is_prime(c: &mut Criterion) {
    let miller_rabin = Fun::new(
        "is_prime",
        |b, _| {
            b.iter(|| (1..N).step_by(STEP).filter(|&n| primal::is_prime(n as u64)).count())
        });
    let sieve = Fun::new(
        "Sieve::is_prime",
        |b, _| {
            let sieve = primal::Sieve::new(N);
            b.iter(|| {
                (1..N).step_by(STEP)
                    .filter(|&n| sieve.is_prime(n)).count()
            })
        });
    let sieve_with_init = Fun::new(
        "Sieve::is_prime with init",
        |b, _| {
            b.iter(|| {
                let sieve = primal::Sieve::new(N);
                (1..N).step_by(STEP)
                    .filter(|&n| sieve.is_prime(n)).count()
            })
        });

    c.bench_functions(
        "is_prime", vec![miller_rabin, sieve, sieve_with_init], ());
}

criterion_group!(benches, is_prime);
criterion_main!(benches);

