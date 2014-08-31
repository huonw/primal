//! Slow and basic handling of primes.

use std::{iter, option, cmp};
use std::collections::{Bitv, bitv};

/// Stores information about primes.
pub struct Primes {
    // This only stores odd numbers, since even numbers are mostly
    // non-prime.
    v: Bitv
}

pub type PrimeIterator<'a> = iter::Chain<
    option::Item<uint>,
    iter::FilterMap<'static, (uint, bool), uint,
       iter::Enumerate<bitv::Bits<'a>>>>;

pub type Factorisation = Vec<(uint, u32)>;

impl Primes {
    /// Construct a `Primes` via a sieve, stopping at `upto`.
    pub fn sieve(upto: uint) -> Primes {
        // bad stuff happens for very small bounds.
        let upto = cmp::max(10, upto);

        let mut is_prime = Bitv::with_capacity((upto + 1) / 2, true);
        // 1 isn't prime
        is_prime.set(0, false);

        // multiples of 3 aren't prime (3 is handled separately, so
        // the ticking works properly)
        for i in iter::range_step(4, upto / 2, 3) {
            is_prime.set(i, false);
        }

        let mut check = 2; // 5
        let mut tick = 1; // step by 2 to get 7
        let bound = (upto as f64).sqrt() as uint + 1;
        while check <= bound {
            if is_prime[check] {
                let p = 2 * check + 1;
                for zero in iter::range_step(2 * check * (check + 1), upto / 2, p) {
                    is_prime.set(zero, false);
                }
            }

            check += tick;
            tick = 3 - tick;
        }

        Primes { v: is_prime }
    }

    /// The largest number stored.
    pub fn upper_bound(&self) -> uint {
        self.v.len() * 2 + 1
    }

    /// Check if `n` is prime, failing if `n` is larger than the upper
    /// bound of this Primes instance.
    pub fn is_prime(&self, n: uint) -> bool {
        assert!(n <= self.upper_bound());
        if n % 2 == 0 {
            // 2 is the evenest prime.
            n == 2
        } else {
            self.v[n / 2]
        }
    }

    /// Iterator over the primes stored in this map.
    pub fn primes<'a>(&'a self) -> PrimeIterator<'a> {
        Some(2).move_iter()
            .chain(self.v.iter().enumerate().filter_map(|(i, is_prime)| {
                if is_prime {
                    Some(2 * i + 1)
                } else {
                    None
                }
            }))
    }

    /// Factorise `n` into (prime, exponent pairs).
    ///
    /// Will fail if n has any factors large than the upper bound of
    /// this `Primes` instance.
    pub fn factor(&self, mut n: uint) -> Vec<(uint, u32)> {
        assert!(n > 0);
        let mut ret = Vec::new();

        for p in self.primes() {
            if n == 1 { break }

            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            if count > 0 {
                ret.push((p,count));
            }
        }
        // FIXME handle errors properly
        assert!(n == 1);
        ret
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Primes;
    use self::test::Bencher;

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
            (11, true)
                ];

        for &(n, expected) in tests.iter() {
            assert_eq!(primes.is_prime(n), expected);
        }
    }

    #[test]
    fn primes_iterator() {
        let primes = Primes::sieve(1000);
        let expected = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        assert_eq!(primes.primes().take(expected.len()).collect::<Vec<uint>>().as_slice(),
                   expected.as_slice());
    }

    #[test]
    fn factor() {
        let primes = Primes::sieve(1000);

        let tests: &[(uint, &[(uint, u32)])] = &[
            (1, &[]),
            (2, &[(2u, 1u32)]),
            (3, &[(3, 1)]),
            (4, &[(2, 2)]),
            (5, &[(5, 1)]),
            (6, &[(2, 1), (3, 1)]),
            (7, &[(7, 1)]),
            (8, &[(2, 3)]),
            (9, &[(3, 2)]),
            (10, &[(2, 1), (5, 1)]),
            ];
        for &(n, expected) in tests.iter() {
            assert_eq!(primes.factor(n), expected.to_vec());
        }
    }

    #[bench]
    fn sieve_small(b: &mut Bencher) {
        b.iter(|| Primes::sieve(100))
    }
    #[bench]
    fn sieve_medium(b: &mut Bencher) {
        b.iter(|| Primes::sieve(10_000))
    }
    #[bench]
    fn sieve_large(b: &mut Bencher) {
        b.iter(|| Primes::sieve(100_000))
    }
}
