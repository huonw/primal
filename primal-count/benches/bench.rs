#[macro_use]
extern crate bencher;

use bencher::Bencher;

use primal_count::primes_below;
use primal_count::PrimeCounter;

const LARGE_VAL: usize = 10_000_000_000;

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
        primes_below(LARGE_VAL)
    });
}

fn d(bench: &mut Bencher) {
    bench.iter(|| {
        let mut pc = PrimeCounter::new(LARGE_VAL);
        pc.primes_below(LARGE_VAL);
    });
}

fn e(bench: &mut Bencher) {
    bench.iter(|| {
        primes_below(LARGE_VAL);
        primes_below(LARGE_VAL >> 4);
    });
}

fn f(bench: &mut Bencher) {
    bench.iter(|| {
        let mut pc = PrimeCounter::new(LARGE_VAL);
        pc.primes_below(LARGE_VAL);
        pc.primes_below(LARGE_VAL >> 4);
    });
}

fn g(bench: &mut Bencher) {
    bench.iter(|| {
        primes_below(LARGE_VAL * 100);
    });
}

fn h(bench: &mut Bencher) {
    bench.iter(|| {
        primes_below(1_000_000_000_000_000);
    });
}



benchmark_group!(benches, a, b, c, d, e, f, g, h);
benchmark_main!(benches);