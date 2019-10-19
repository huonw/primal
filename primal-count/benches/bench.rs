#[macro_use]
extern crate criterion;
extern crate primal_count;
use primal_count::primes_below;
use primal_count::PrimeCounter;
use criterion::{Criterion, ParameterizedBenchmark};

const SIZES: [usize; 6] = [100, 10_000, 100_000, 1_000_000, 10_000_000, 10_000_000_000];

macro_rules! create_benchmarks {
    ($(
        fn $group_id: ident($input: expr) {
            $first_name: expr => $first_func: expr,
            $($rest_name: expr => $rest_func: expr,)*
        }
    )*) => {
        $(
            fn $group_id(c: &mut Criterion) {
                let input = $input;

                let bench = ParameterizedBenchmark::new(
                    $first_name, $first_func, input.into_iter().cloned())
                    $( .with_function($rest_name, $rest_func) )*;
                c.bench(stringify!($group_id), bench);
            }
        )*
    }
}

create_benchmarks! {
    fn new(SIZES) {
        "PrimeCounter" => |b, upto: &usize| b.iter(|| PrimeCounter::new(*upto)),
    }

    fn prime_pi(SIZES) {
        "PrimeCounter" => |b, upto: &usize| {
            let mut s = PrimeCounter::new(*upto + 1);
            b.iter(|| s.primes_below(*upto));
        },
  
        "PrimeCounter with init" => |b, upto: &usize| {
            b.iter(|| {
                let mut s = PrimeCounter::new(*upto + 1);
                s.primes_below(*upto)
                });
        },
    }
}

criterion_group!(benches, new, prime_pi);
criterion_main!(benches);
