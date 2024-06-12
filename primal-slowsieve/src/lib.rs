//! Sieve small numbers.
//!
//! This is designed to be used via the `primal` crate.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use core::{cmp, iter};
use primal_bit::BitVec;

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

/// Stores information about primes up to some limit.
///
/// This uses at least `limit / 16 + O(1)` bytes of storage.
#[derive(Debug)]
pub struct Primes {
    // This only stores odd numbers, since even numbers are mostly
    // non-prime.
    //
    // This indicates which numbers are composite.
    v: BitVec,
}

/// Iterator over the primes stored in a sieve.
#[derive(Clone)]
pub struct PrimeIterator<'a> {
    two: bool,
    iter: iter::Enumerate<primal_bit::Iter<'a>>,
}

impl Primes {
    /// Construct a `Primes` via a sieve up to at least `limit`.
    ///
    /// This stores all primes less than `limit` (and possibly some
    /// more), allowing for very efficient iteration and primality
    /// testing below this, and guarantees that all numbers up to
    /// `limit^2` can be factorised.
    pub fn sieve(limit: usize) -> Primes {
        // having this out-of-line like this is faster (130 us/iter
        // vs. 111 us/iter on sieve_large), and using a manual while
        // rather than a `range_step` is a similar speedup.
        #[inline(never)]
        fn filter(is_prime: &mut BitVec, limit: usize, p: usize) {
            let mut zero = p * p / 2;
            while zero < limit / 2 {
                is_prime.set(zero, true);
                zero += p;
            }
        }

        // bad stuff happens for very small bounds.
        let limit = cmp::max(10, limit);

        let mut is_prime = BitVec::from_elem((limit + 1) / 2, false);
        // 1 isn't prime
        is_prime.set(0, true);

        // multiples of 3 aren't prime (3 is handled separately, so
        // the ticking works properly)
        filter(&mut is_prime, limit, 3);

        #[cfg(feature = "std")]
        let bound = (limit as f64).sqrt() as usize + 1;
        #[cfg(feature = "libm")]
        let bound = libm::sqrt(limit as f64) as usize + 1;
        // skip 2.
        let mut check = 2;
        let mut tick = if check % 3 == 1 { 2 } else { 1 };

        while check <= bound {
            if !is_prime[check] {
                filter(&mut is_prime, limit, 2 * check + 1)
            }

            check += tick;
            tick = 3 - tick;
        }

        Primes { v: is_prime }
    }

    /// The largest number stored.
    pub fn upper_bound(&self) -> usize {
        self.v.len() * 2
    }

    /// Check if `n` is prime, possibly failing if `n` is larger than
    /// the upper bound of this Primes instance.
    pub fn is_prime(&self, n: usize) -> bool {
        if n % 2 == 0 {
            // 2 is the evenest prime.
            n == 2
        } else {
            assert!(n <= self.upper_bound());
            !self.v[n / 2]
        }
    }

    /// Iterator over the primes stored in this map.
    pub fn primes(&self) -> PrimeIterator<'_> {
        PrimeIterator {
            two: true,
            iter: self.v.iter().enumerate(),
        }
    }

    /// Factorise `n` into (prime, exponent) pairs.
    ///
    /// Returns `Err((leftover, partial factorisation))` if `n` cannot
    /// be fully factored, or if `n` is zero (`leftover == 0`). A
    /// number can not be completely factored if and only if the prime
    /// factors of `n` are too large for this sieve, that is, if there
    /// is
    ///
    /// - a prime factor larger than `U^2`, or
    /// - more than one prime factor between `U` and `U^2`
    ///
    /// where `U` is the upper bound of the primes stored in this
    /// sieve.
    ///
    /// Notably, any number between `U` and `U^2` can always be fully
    /// factored, since these numbers are guaranteed to only have zero
    /// or one prime factors larger than `U`.
    pub fn factor(
        &self,
        mut n: usize,
    ) -> Result<Vec<(usize, usize)>, (usize, Vec<(usize, usize)>)> {
        if n == 0 {
            return Err((0, vec![]));
        }

        let mut ret = Vec::new();

        for p in self.primes() {
            if n == 1 {
                break;
            }

            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            if count > 0 {
                ret.push((p, count));
            }
        }
        if n != 1 {
            let b = self.upper_bound();
            if b * b >= n {
                // n is not divisible by anything from 1..=sqrt(n), so
                // must be prime itself! (That is, even though we
                // don't know this prime specifically, we can infer
                // that it must be prime.)
                ret.push((n, 1));
            } else {
                // large factors :(
                return Err((n, ret));
            }
        }
        Ok(ret)
    }

    /// Count the primes upto and including `n`.
    ///
    /// # Panics
    ///
    /// `count_upto` panics if `n > self.upper_bound()`.
    pub fn count_upto(&self, n: usize) -> usize {
        if n < 2 {
            return 0;
        }

        assert!(n <= self.upper_bound());
        let bit = (n + 1) / 2;
        1 + (bit - self.v.count_ones_before(bit))
    }
}

impl<'a> Iterator for PrimeIterator<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.two {
            self.two = false;
            Some(2)
        } else {
            for (i, is_not_prime) in &mut self.iter {
                if !is_not_prime {
                    return Some(2 * i + 1);
                }
            }
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut iter = self.clone();
        // TODO: this doesn't run in constant time, is it super-bad?
        match (iter.next(), iter.next_back()) {
            (Some(lo), Some(hi)) => {
                let (below_hi, above_hi) = primal_estimate::prime_pi(hi as u64);
                let (below_lo, above_lo) = primal_estimate::prime_pi(lo as u64);

                (
                    (below_hi - cmp::min(above_lo, below_hi)) as usize,
                    Some((above_hi - below_lo + 1) as usize),
                )
            }
            (Some(_), None) => (1, Some(1)),
            (None, _) => (0, Some(0)),
        }
    }
}

impl<'a> DoubleEndedIterator for PrimeIterator<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<usize> {
        loop {
            match self.iter.next_back() {
                Some((i, false)) => return Some(2 * i + 1),
                Some((_, true)) => { /* continue */ }
                None if self.two => {
                    self.two = false;
                    return Some(2);
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Primes;

    #[test]
    fn is_prime() {
        let primes = Primes::sieve(1000);
        let tests = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, false),
            (7, true),
            (8, false),
            (9, false),
            (10, false),
            (11, true),
        ];

        for &(n, expected) in tests.iter() {
            assert_eq!(primes.is_prime(n), expected);
        }
    }

    #[test]
    fn upper_bound() {
        for i in 1..1000 {
            let primes = Primes::sieve(i);
            assert!(primes.upper_bound() >= i);
        }

        let range = if cfg!(feature = "slow_tests") {
            1..200
        } else {
            100..120
        };
        for i in range {
            let i = i * 10000;
            let primes = Primes::sieve(i);
            assert!(primes.upper_bound() >= i);
        }
    }

    #[test]
    fn primes_iterator() {
        let primes = Primes::sieve(50);
        let mut expected = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        assert_eq!(primes.primes().collect::<Vec<usize>>(), expected);

        expected.reverse();
        assert_eq!(primes.primes().rev().collect::<Vec<usize>>(), expected);
    }

    #[test]
    fn factor() {
        let primes = Primes::sieve(1000);

        let tests: &[(usize, &[(usize, usize)])] = &[
            (1, &[]),
            (2, &[(2_usize, 1)]),
            (3, &[(3, 1)]),
            (4, &[(2, 2)]),
            (5, &[(5, 1)]),
            (6, &[(2, 1), (3, 1)]),
            (7, &[(7, 1)]),
            (8, &[(2, 3)]),
            (9, &[(3, 2)]),
            (10, &[(2, 1), (5, 1)]),
            (2 * 2 * 2 * 2 * 2 * 3 * 3 * 3 * 3 * 3, &[(2, 5), (3, 5)]),
            (
                2 * 3 * 5 * 7 * 11 * 13 * 17 * 19,
                &[
                    (2, 1),
                    (3, 1),
                    (5, 1),
                    (7, 1),
                    (11, 1),
                    (13, 1),
                    (17, 1),
                    (19, 1),
                ],
            ),
            // a factor larger than that stored in the map
            (7561, &[(7561, 1)]),
            (2 * 7561, &[(2, 1), (7561, 1)]),
            (4 * 5 * 7561, &[(2, 2), (5, 1), (7561, 1)]),
        ];
        for &(n, expected) in tests.iter() {
            assert_eq!(primes.factor(n), Ok(expected.to_vec()));
        }
    }

    #[test]
    fn factor_compare() {
        let short = Primes::sieve(30);
        let long = Primes::sieve(10000);

        let short_lim = short.upper_bound() * short.upper_bound() + 1;

        // every number less than bound^2 can be factored (since they
        // always have a factor <= bound).
        for n in 0..short_lim {
            assert_eq!(short.factor(n), long.factor(n))
        }
        // larger numbers can only sometimes be factored
        'next_n: for n in short_lim..10000 {
            let possible = short.factor(n);
            let real = long.factor(n).ok().unwrap();

            let mut seen_small = None;
            for (this_idx, &(p, i)) in real.iter().enumerate() {
                let last_short_prime = if p >= short_lim {
                    this_idx
                } else if p > short.upper_bound() {
                    match seen_small {
                        Some(idx) => idx,
                        None if i > 1 => this_idx,
                        None => {
                            // we can cope with one
                            seen_small = Some(this_idx);
                            continue;
                        }
                    }
                } else {
                    // small enough
                    continue;
                };

                // break into the two parts
                let (low, hi) = real.split_at(last_short_prime);
                let leftover = hi.iter().fold(1, |x, &(p, i)| x * p.pow(i as u32));

                assert_eq!(possible, Err((leftover, low.to_vec())));
                continue 'next_n;
            }

            // if we're here, we know that everything should match
            assert_eq!(possible, Ok(real))
        }
    }

    #[test]
    fn factor_failures() {
        let primes = Primes::sieve(30);

        assert_eq!(primes.factor(0), Err((0, vec![])));
        // can only handle one large factor
        assert_eq!(primes.factor(31 * 31), Err((31 * 31, vec![])));
        assert_eq!(
            primes.factor(2 * 3 * 31 * 31),
            Err((31 * 31, vec![(2, 1), (3, 1)]))
        );

        // prime that's too large (bigger than 30*30).
        assert_eq!(primes.factor(7561), Err((7561, vec![])));
        assert_eq!(
            primes.factor(2 * 3 * 7561),
            Err((7561, vec![(2, 1), (3, 1)]))
        );
    }

    #[test]
    fn size_hint() {
        let mut i = 0;
        while i < 1000 {
            let sieve = Primes::sieve(i);

            let mut primes = sieve.primes();

            // check the size hint at each and every iteration
            loop {
                let (lo, hi) = primes.size_hint();

                let copy = primes.clone();
                let len = copy.count();

                let next = primes.next();

                assert!(
                    lo <= len && len <= hi.unwrap(),
                    "found failing size_hint for {:?} to {}, should satisfy: {} <= {} <= {:?}",
                    next,
                    i,
                    lo,
                    len,
                    hi
                );

                if next.is_none() {
                    break;
                }
            }

            i += 100;
        }
    }

    #[test]
    fn count_upto() {
        let (limit, mult) = if cfg!(feature = "slow_tests") {
            (2_000_000, 19_998)
        } else {
            (200_000, 1_998)
        };
        let sieve = Primes::sieve(limit);

        for i in (0..20).chain((0..100).map(|n| n * mult + 1)) {
            let val = sieve.count_upto(i);
            let true_ = sieve.primes().take_while(|p| *p <= i).count();
            assert!(
                val == true_,
                "failed for {}, true {}, computed {}",
                i,
                true_,
                val
            )
        }
    }
}
