#[macro_use]
extern crate criterion;

use primal_slowsieve::Primes;
use criterion::Criterion;

const SIZES: [usize; 4] = [100, 10_000, 100_000, 1_000_000];

fn sieve(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Primes::sieve",
        |b, upto: &&usize| b.iter(|| Primes::sieve(**upto)),
        &SIZES);
}

fn primes(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Primes::primes",
        |b, upto: &&usize| {
            let s = Primes::sieve(**upto);
            b.iter(|| s.primes().count())
        },
        &SIZES);
}

criterion_group!(benches, sieve, primes);
criterion_main!(benches);
