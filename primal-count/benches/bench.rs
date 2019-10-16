#[macro_use]
extern crate bencher;

use bencher::Bencher;

use primal_count::primes_below;

fn a(bench: &mut Bencher) {
    bench.iter(|| {
        primes_below(7)
    })
}

fn b(bench: &mut Bencher) {
    bench.iter(|| {
        primes_below(1_000)
    });
}

fn c(bench: &mut Bencher) {
    bench.iter(|| {
        primes_below(10_000_000_000)
    });
}

benchmark_group!(benches, a, b, c);
benchmark_main!(benches);