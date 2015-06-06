//! `primal` puts raw power into prime numbers.
//!
//! This crates includes
//!
//! - optimised prime sieves
//! - checking for primality
//! - enumerating primes
//! - factorising numbers
//! - estimating upper and lower bounds for π(*n*) (the number of primes
//!   below *n*) and *p<sub>k</sub>* (the <i>k</i>th prime)
//!
//! This uses a state-of-the-art cache-friendly Sieve of Eratosthenes
//! to enumerate the primes up to some fixed bound (in a memory
//! efficient manner), and then allows this cached information to be
//! used for things like enumerating and counting primes.
//!
//! `primal` takes around 2.8 seconds and less than 3MB of RAM to
//! count the exact number of primes below 10<sup>10</sup> (455052511)
//! on the author's laptop (i7-3517U).
//!
//! [*Source*](http://github.com/huonw/primal)
//!
//! # Using this library
//!
//! Just add the following to your [`Cargo.toml`](http://crates.io/):
//!
//! ```toml
//! [dependencies]
//! primal = "0.2"
//! ```
//!
//! # Example
//!
//! Let's find the 10001st prime. The easiest way is to enumerate the
//! primes, and find the 10001st:
//!
//! ```rust
//! // (.nth is zero indexed.)
//! let p = primal::Primes::all().nth(10001 - 1).unwrap();
//! println!("The 10001st prime is {}", p); // 104743
//! ```
//!
//! `Primes` is flexible at the cost of performance, so if we wish to
//! optimise we could instead explicitly sieve and then find the
//! appropriate prime.  Unfortunately, `Sieve` requires a limit to
//! know how far to sieve: we need some way to find an upper bound to
//! be guaranteed to be at least as large the 10001st prime. We could
//! guess that, say, 10<sup>8</sup> will be large enough and use that,
//! but that's probably a huge overestimate. We could also try
//! filtering with exponentially larger upper bounds until we find one
//! that works (e.g. doubling each time), or... we could just take a
//! shortcut and use deeper mathematics via
//! [`estimate_nth_prime`](fn.estimate_nth_prime.html).
//!
//! ```rust
//! // find our upper bound
//! let (_lo, hi) = primal::estimate_nth_prime(10001);
//!
//! // find the primes up to this upper bound
//! let sieve = primal::Sieve::new(hi as usize);
//!
//! let p = sieve.primes_from(0).nth(10001 - 1).unwrap();
//! println!("The 10001st prime is {}", p); // 104743
//! ```
//!

#![cfg_attr(all(test, feature = "unstable"), feature(test, step_by))]

extern crate primal_estimate;
extern crate primal_check;
extern crate primal_sieve;

#[cfg(all(test, feature = "unstable"))] extern crate test;

pub use primal_estimate::prime_pi as estimate_prime_pi;
pub use primal_estimate::nth_prime as estimate_nth_prime;
pub use primal_check::miller_rabin as is_prime;
pub use primal_check::{as_perfect_power, as_prime_power};

pub use primal_sieve::{StreamingSieve, Sieve, Primes};


#[cfg(all(test, feature = "unstable"))]
mod benches {
    extern crate test;

    use super::{Sieve, is_prime};
    use self::test::Bencher;


    const N: usize = 1_000_000;
    const STEP: usize = 101;
    #[bench]
    fn bench_miller_rabin_tests(b: &mut Bencher) {
        b.iter(|| {
            (1..N).step_by(STEP)
                .filter(|&n| is_prime(n as u64)).count()
        })
    }
    #[bench]
    fn bench_sieve_tests(b: &mut Bencher) {
        b.iter(|| {
            let sieve = Sieve::new(1_000_000);
            (1..N).step_by(STEP)
                .filter(|&n| sieve.is_prime(n)).count()
        })
    }
}
