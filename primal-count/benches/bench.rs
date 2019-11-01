#[macro_use]
extern crate criterion;
extern crate primal_count;
use criterion::{Criterion, ParameterizedBenchmark};
use primal_count::PrimeCounter;

const SIZES: [usize; 7] = [
    100,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    10_000_000_000,
    100_000_000_000,
];

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
            b.iter(|| s.prime_pi(*upto));
        },

        "PrimeCounter with init" => |b, upto: &usize| {
            b.iter(|| {
                let mut s = PrimeCounter::new(*upto + 1);
                s.prime_pi(*upto)
                });
        },
    }
}

criterion_group!(benches, new, prime_pi);
criterion_main!(benches);
