//! Simplistic and relatively unoptimised handling of basic
//! tasks around primes:
//!
//! - checking for primality
//! - enumerating primes
//! - factorising numbers
//! - estimating upper and lower bounds for π(*n*) (the number of primes
//!   below *n*) and *p<sub>k</sub>* (the <i>k</i>th prime)
//!
//! This uses a basic Sieve of Eratosthenes to enumerate the primes up to
//! some fixed bound (in a relatively memory efficient manner), and then
//! allows this cached information to be used for things like enumerating
//! the primes, and factorisation via trial division.
//!
//! (Despite the name, it can sieve the primes up to 10<sup>9</sup> in
//! about 5 seconds.)
//!
//! [*Source*](http://github.com/huonw/slow_primes)
//!
//! # Example
//!
//! Let's find the 10001st prime. The basic idea is to enumerate the
//! primes and then take the 10001st in that list.
//!
//! Unfortunately, `Primes::sieve` takes an upper bound, and it gives
//! us no information beyond this; so we really need some way to find
//! an upper bound to be guaranteed to include the 10001st prime. If
//! we had an a priori number we could just use that, but we don't
//! (for the purposes of this example, anyway). Hence, we can either
//! try filtering with exponentially larger upper bounds until we find
//! one that works (e.g. doubling each time), or just take a shortcut
//! and use deeper mathematics via
//! [`estimate_nth_prime`](fn.estimate_nth_prime.html).
//!
//! ```rust
//! // find our upper bound
//! let (_lo, hi) = slow_primes::estimate_nth_prime(10001);
//!
//! // find the primes up to this upper bound
//! let sieve = slow_primes::Primes::sieve(hi as uint);
//!
//! // (.nth is zero indexed.)
//! match sieve.primes().nth(10001 - 1) {
//!     Some(p) => println!("The 10001st prime is {}", p), // 104743
//!     None => unreachable!(),
//! }
//! ```
//!
//! # Using this library
//!
//! Just add the following to your [`Cargo.toml`](http://crates.io/):
//!
//! ```toml
//! [dependencies.slow_primes]
//! git = "https://github.com/huonw/slow_primes"
//! ```

extern crate "num" as num_;

#[cfg(test)] extern crate test;

pub use estimate::{estimate_prime_pi, estimate_nth_prime};
//pub use fast_sieve::Sieve;
pub use is_prime::{is_prime_miller_rabin};
pub use perfect_power::{as_perfect_power, as_prime_power};
pub use sieve::{Primes, PrimeIterator};

mod estimate;
mod fast_sieve;
mod is_prime;
mod perfect_power;
mod sieve;

#[allow(dead_code)]
mod tables;

/// (prime, exponent) pairs storing the prime factorisation of a
/// number.
pub type Factors = Vec<(uint, uint)>;

#[cfg(test)]
mod tests {
    extern crate test;

    use std::iter::range_step;
    use super::{Primes, is_prime_miller_rabin};
    use self::test::Bencher;


    const N: uint = 1_000_000;
    const STEP: uint = 101;
    #[bench]
    fn bench_miller_rabin_tests(b: &mut Bencher) {
        b.iter(|| {
            range_step(1, N, STEP)
                .filter(|&n| is_prime_miller_rabin(n as u64)).count()
        })
    }
    #[bench]
    fn bench_sieve_tests(b: &mut Bencher) {
        b.iter(|| {
            let sieve = Primes::sieve(1_000_000);
            range_step(1, N, STEP)
                .filter(|&n| sieve.is_prime(n)).count()
        })
    }
}
