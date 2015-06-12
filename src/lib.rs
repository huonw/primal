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
//! on my laptop (i7-3517U).
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
//! # Examples
//!
//! ## "Indexing" Primes
//!
//! Let's find the 10001st prime. The easiest way is to enumerate the
//! primes, and find the 10001st:
//!
//! ```rust
//! // (.nth is zero indexed.)
//! let p = primal::Primes::all().nth(10001 - 1).unwrap();
//! println!("The 10001st prime is {}", p); // 104743
//! # assert_eq!(p, 104743);
//! ```
//!
//! This takes around 400 microseconds on my computer, which seems
//! nice and quick, but, `Primes` is flexible at the cost of
//! performance: we can make it faster. The `StreamingSieve` type
//! offers a specialised `nth_prime` function:
//!
//! ```rust
//! let p = primal::StreamingSieve::nth_prime(10001);
//! println!("The 10001st prime is {}", p); // 104743
//! # assert_eq!(p, 104743);
//! ```
//!
//! This runs in only 10 microseconds! `StreamingSieve` is extremely
//! efficient and uses very little memory. It is the best way to solve
//! this task with `primal`.
//!
//! Since that was so easy, let's now make the problem bigger and
//! harder: find the sum of the 100,000th, 200,000th, 300,000th, ...,
//! 10,000,000th primes (100 in total).
//!
//! We could call `StreamingSieve::nth_prime` repeatedly:
//!
//! ```rust,no_run
//! // the primes we want to find
//! let ns = (1..100 + 1).map(|x| x * 100_000).collect::<Vec<_>>();
//!
//! // search and sum them up
//! let sum = ns.iter()
//!             .map(|n| primal::StreamingSieve::nth_prime(*n))
//!             .fold(0, |a, b| a + b);
//! println!("the sum is {}", sum);
//! ```
//!
//! This takes around 1.6s seconds to print `the sum is 8795091674`;
//! not so speedy. Each call to `nth_prime` is individually fast (400
//! microseconds for 100,000 to 40 milliseconds for 10,000,000) but
//! they add up to something bad. Every one is starting from the start
//! and redoing work that previous calls have done... wouldn't it be
//! nice if we could just do the computation for 10,000,000 and reuse
//! that for the smaller ones?
//!
//! The `Sieve` type is a wrapper around `StreamingSieve` that
//! caches information, allowing repeated queries to be answered
//! efficiently.
//!
//! There's one hitch: `Sieve` requires a limit to know how far to
//! sieve: we need some way to find an upper bound to be guaranteed to
//! be at least as large as all our primes. We could guess that, say,
//! 10<sup>10</sup> will be large enough and use that, but that's a
//! huge overestimate (spoilers: the 10,000,000th prime is around
//! 2&times;10<sup>8</sup>). We could also try filtering with
//! exponentially larger upper bounds until we find one that works
//! (e.g. doubling each time), or, we could just take a shortcut and
//! use deeper mathematics via
//! [`estimate_nth_prime`](fn.estimate_nth_prime.html).
//!
//! ```rust
//! // the primes we want to find
//! let ns = (1..100 + 1).map(|x| x * 100_000).collect::<Vec<_>>();
//!
//! // find our upper bound
//! let (_lo, hi) = primal::estimate_nth_prime(10_000_000);
//!
//! // find the primes up to this upper bound
//! let sieve = primal::Sieve::new(hi as usize);
//!
//! // now we can efficiently sum them up
//! let sum = ns.iter()
//!             .map(|n| sieve.nth_prime(*n))
//!             .fold(0, |a, b| a + b);
//! println!("the sum is {}", sum);
//! # assert_eq!(sum, 8795091674);
//! ```
//!
//! This takes around 40 milliseconds, and gives the same output: much
//! better!
//!
//! (By the way, the version using 10<sup>10</sup> as the bound
//! instead of the more accurate estimate still only takes ~3
//! seconds.)
//!
//! ## Counting Primes
//!
//! Another problem: count the number of primes below 1 million. This
//! is evaluating the [prime-counting function
//! π](https://en.wikipedia.org/wiki/Prime-counting_function),
//! i.e. π(10<sup>6</sup>).
//!
//! As above, there's a few ways to attack this: the iterator, and the
//! sieves.
//!
//! ```rust
//! const LIMIT: usize = 1_000_000;
//!
//! // iterator
//! let count = primal::Primes::all().take_while(|p| *p < LIMIT).count();
//! println!("there are {} primes below 1 million", count); // 78498
//! # assert_eq!(count, 78498);
//!
//! // sieves
//! let sieve = primal::Sieve::new(LIMIT);
//! let count = sieve.prime_pi(LIMIT);
//! println!("there are {} primes below 1 million", count);
//! # assert_eq!(count, 78498);
//!
//! let count = primal::StreamingSieve::prime_pi(LIMIT);
//! println!("there are {} primes below 1 million", count);
//! # assert_eq!(count, 78498);
//! ```
//!
//! `StreamingSieve` is fastest (380 microseconds) followed by `Sieve`
//! (400) with `Primes` bringing up the rear at 1300 microseconds. Of
//! course, repeated queries will be faster with `Sieve` than with
//! `StreamingSieve`, but that flexibility comes at the cost of extra
//! memory use.
//!
//! If an approximation is all that is required, `estimate_prime_pi`
//! provides close upper and lower bounds:
//!
//! ```rust
//! let (lo, hi) = primal::estimate_prime_pi(1_000_000);
//! println!("there are between {} and {} primes below 1 million", lo, hi);
//! // 78304, 78573
//! # assert_eq!(lo, 78304);
//! # assert_eq!(hi, 78573);
//! ```
//!
//! ## Searching Primes
//!
//! Now for something where `Primes` might be useful: find the first
//! prime where the binary expansion (not including trailing zeros)
//! ends like `00..001` with at least 27 zeros. This condition is
//! checked by:
//!
//! ```rust
//! fn check(p: usize) -> bool {
//!     p > 1 && (p / 2).trailing_zeros() >= 27
//! }
//! ```
//!
//! I have no idea how large the prime might be: I know it's
//! guaranteed to be at *least* 2<sup>27 + 1</sup> + 1, but not an
//! upper limit.
//!
//! The `Primes` iterator works perfectly for this:
//!
//! ```rust
//! # fn check(p: usize) -> bool {  p > 1 && (p / 2).trailing_zeros() >= 5 } // 27 is too slow
//! let p = primal::Primes::all().find(|p| check(*p)).unwrap();
//! println!("the prime is {}", p);
//! # assert_eq!(p, 193);
//! ```
//!
//! It takes about 3.1 seconds for my computer to spit out 3,221,225,473.
//!
//! Using a sieve is a little trickier: one approach is to start with
//! some estimated upper bound (like double the absolute lower bound),
//! look for a valid prime. If one isn't found, double the upper bound
//! and start again. The `primes_from` method allows for saving a
//! little bit of work: we can start iterating from an arbitrary point
//! in the sequence, such as the lower bound.
//!
//! ```rust
//! # fn check(p: usize) -> bool {  p > 1 && (p / 2).trailing_zeros() >= 5 } // 27 is too slow
//! let p;
//! let mut lower_bound = 1 << (27 + 1);
//! # let mut lower_bound = 1 << (5 + 1);
//! loop {
//!     // our upper bound is double the lower bound
//!     let sieve = primal::Sieve::new(lower_bound * 2);
//!     if let Some(p_) = sieve.primes_from(lower_bound).find(|p| check(*p)) {
//!         p = p_;
//!         break
//!     }
//!     lower_bound *= 2;
//! }
//! println!("the prime is {}", p);
//! # assert_eq!(p, 193);
//! ```
//!
//! This takes around 3.5 seconds to print the same number. Slower
//! than the iterator!
//!
//! I was just using this silly condition as an example of something
//! that doesn't have an obvious upper bound, rather than a problem
//! that is hard to do fast. There's a much faster way to tackle it,
//! by inverting the problem: construct numbers that satisfy `check`,
//! and check the primality of those.
//!
//! The numbers that satisfy `check` are `k * (1 << (27 + 1)) + 1` for
//! `k >= 1`, so the only hard bit is testing primality. Fortunately,
//! `primal` offers the `is_prime` function which is an efficient way
//! to do primality tests, even of very large numbers.
//!
//! ```rust
//! let mut p = 0;
//! for k in 1.. {
//!     p = k * (1 << (27 + 1)) + 1;
//!     if primal::is_prime(p) { break }
//! }
//! println!("the prime is {}", p);
//! # assert_eq!(p, 3_221_225_473);
//! ```
//!
//! This takes 6 <em>micro</em>seconds: more than 500,000&times;
//! faster than the iterator!

#![cfg_attr(all(test, feature = "unstable"), feature(test, step_by))]

extern crate primal_estimate;
extern crate primal_check;
extern crate primal_sieve;

#[cfg(all(test, feature = "unstable"))] extern crate test;

pub use primal_estimate::prime_pi as estimate_prime_pi;
pub use primal_estimate::nth_prime as estimate_nth_prime;
pub use primal_check::miller_rabin as is_prime;
pub use primal_check::{as_perfect_power, as_prime_power};

pub use primal_sieve::{StreamingSieve, Sieve, SievePrimes, Primes};


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
