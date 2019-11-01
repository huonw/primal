#[macro_use]
extern crate criterion;
extern crate primal;
use criterion::{Criterion, Fun, ParameterizedBenchmark};

const N: usize = 1_000_000;
const STEP: usize = 101;
const SIZES: [usize; 8] = [100, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000, 10_000_000_000];

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

create_benchmarks!{
    fn prime_pi(SIZES) {
        "PrimeCounter" => |b, upto: &usize| {
            let mut s = primal::PrimeCounter::new(*upto + 1);
            b.iter(|| s.prime_pi(*upto));
        },
        "Sieve" => |b, upto: &usize| {
            let s = primal::Sieve::new(*upto + 1);
            b.iter(|| s.prime_pi(*upto));
        },

        "PrimeCounter with init" => |b, upto: &usize| {
            b.iter(|| {
                let mut s = primal::PrimeCounter::new(*upto + 1);
                s.prime_pi(*upto)
            });
        },
        "Sieve with init" => |b, upto: &usize| {
            b.iter(|| {
                let s = primal::Sieve::new(*upto + 1);
                s.prime_pi(*upto)
            });
        },

        "StreamingSieve" => |b, upto: &usize| {
            b.iter(|| primal::StreamingSieve::prime_pi(*upto))
        },
        "Primes" => |b, upto: &usize| {
            b.iter(|| primal::Primes::all().take_while(|x| *x <= *upto).count())
        },
    }
}

criterion_group!(benches, is_prime, prime_pi);
criterion_main!(benches);
