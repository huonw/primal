#[macro_use]
extern crate criterion;
extern crate primal_sieve;
extern crate primal_estimate;
use primal_sieve::{StreamingSieve, Sieve, Primes};
use criterion::{Criterion, ParameterizedBenchmark};

const SIZES: [usize; 5] = [100, 10_000, 100_000, 1_000_000, 10_000_000];

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
                    $first_name, $first_func, input.iter().copied())
                    $( .with_function($rest_name, $rest_func) )*;
                c.bench(stringify!($group_id), bench);
            }
        )*
    }
}

create_benchmarks! {
    fn new(SIZES) {
        "Sieve" => |b, upto: &usize| b.iter(|| Sieve::new(*upto)),
    }

    fn prime_pi(SIZES) {
        "Sieve" => |b, upto: &usize| {
            let s = Sieve::new(*upto + 1);
            b.iter(|| s.prime_pi(*upto));
        },
        "Sieve with init" => |b, upto: &usize| {
            b.iter(|| {
                let s = Sieve::new(*upto + 1);
                s.prime_pi(*upto)
            });
        },

        "StreamingSieve" => |b, upto: &usize| {
            b.iter(|| StreamingSieve::prime_pi(*upto))
        },
        "Primes" => |b, upto: &usize| {
            b.iter(|| Primes::all().take_while(|x| *x <= *upto).count())
        },
    }

    fn nth_prime([100, 10_000, 100_000, 1_000_000]) {
        "Sieve" => |b, n: &usize| {
            let (_, hi) = primal_estimate::nth_prime(*n as u64);
            let s = Sieve::new(hi as usize);
            b.iter(|| s.nth_prime(*n));
        },
        "Sieve with init" => |b, n: &usize| {
            b.iter(|| {
                let (_, hi) = primal_estimate::nth_prime(*n as u64);
                let s = Sieve::new(hi as usize);
                s.nth_prime(*n)
            });
        },
        "StreamingSieve" => |b, n: &usize| {
            b.iter(|| StreamingSieve::nth_prime(*n))
        },
        "Primes" => |b, n: &usize| {
            b.iter(|| Primes::all().nth(*n - 1).unwrap())
        },
    }

    fn iterate(SIZES) {
        "Sieve" => |b, upto: &usize| {
            let s = Sieve::new(*upto);
            b.iter(|| s.primes_from(0).count());
        },
        "Sieve with init" => |b, upto: &usize| {
            b.iter(|| {
                let s = Sieve::new(*upto);
                s.primes_from(0).count()
            });
        },
        "Primes" => |b, upto: &usize| {
            b.iter(|| Primes::all().take_while(|p| *p < *upto).count())
        },
    }

    fn factor([131, 7561, 65521, 1048573, 2*3*5*7*11*13*17*19]) {
        "Sieve" => |b, n: &usize| {
            let s = Sieve::new(0x10000);

            b.iter(|| s.factor(*n).ok());
        },
    }
}

criterion_group!(benches, new, prime_pi, nth_prime, factor, iterate);
criterion_main!(benches);
