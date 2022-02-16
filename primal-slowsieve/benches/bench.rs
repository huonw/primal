#[macro_use]
extern crate criterion;

use criterion::Criterion;
use primal_slowsieve::Primes;

const SIZES: [usize; 4] = [100, 10_000, 100_000, 1_000_000];

fn sieve(c: &mut Criterion) {
    let mut group = c.benchmark_group("Primes::sieve");
    for i in &SIZES {
        group.bench_with_input(i.to_string(), i, |b, upto: &usize| {
            b.iter(|| Primes::sieve(*upto))
        });
    }
    group.finish();
}

fn primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Primes::primes");
    for i in &SIZES {
        group.bench_with_input(i.to_string(), i, |b, upto: &usize| {
            let s = Primes::sieve(*upto);
            b.iter(|| s.primes().count())
        });
    }
    group.finish();
}

criterion_group!(benches, sieve, primes);
criterion_main!(benches);
