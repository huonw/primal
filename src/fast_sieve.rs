#![allow(dead_code)]

use primal_bit::{BitVec};
use std::{cmp};

use primal_smallsieve::Primes;

/// A segmented sieve that yields only a small run of primes at a
/// time.
///
/// This is heavily inspired by this [segmented
/// sieve](http://primesieve.org/segmented_sieve.html) code.
#[derive(Debug)]
pub struct StreamingSieve {
    small: Primes,
    // stores which numbers *aren't* prime, i.e. true == composite.
    sieve: BitVec,
    primes: Vec<(usize, usize)>,

    low: usize,
    current: usize,
    limit: usize,
}

const CACHE: usize = 32 << 10;
// 8 for the bit vector, 2 for storing odd numbers only
const SEG_ELEMS: usize = 8 * CACHE;
const SEG_LEN: usize = 2 * SEG_ELEMS;

impl StreamingSieve {
    /// Create a new instance of the streaming sieve that will
    /// correctly progressively filter primes up to `limit`.
    pub fn new(limit: usize) -> StreamingSieve {
        let small = Primes::sieve((limit as f64).sqrt() as usize + 1);
        let current = 3;
        let low = 0;

        let elems = cmp::min(limit, SEG_ELEMS);
        StreamingSieve {
            small: small,
            sieve: BitVec::from_elem(elems, false),
            primes: vec![],

            low: low,
            current: current,
            limit: limit
        }
    }

    /// Extract the next chunk of filtered primes, the return value is
    /// `Some((low, v))` or `None` if the sieve has reached the limit.
    ///
    /// The vector stores bits for each odd number starting at `low`.
    /// Bit `n` of `v` is set if and only if `low + 2 * n + 1` is
    /// prime.
    ///
    /// NB. the prime 2 is not included in any of these sieves and so
    /// needs special handling.
    pub fn next(&mut self) -> Option<(usize, &BitVec)> {
        if self.low >= self.limit {
            return None
        }

        let low = self.low;
        self.low += SEG_LEN;
        let high = cmp::min(low + SEG_LEN - 1, self.limit);
        self.sieve.clear();

        let mut s = self.current;

        while s * s <= high {
            if self.small.is_prime(s) {
                self.primes.push((s, s * s - low));
            }
            s += 1
        }

        self.current = s;
        let top = cmp::min(SEG_LEN, self.limit) / 2;
        for &mut (k, ref mut next) in self.primes.iter_mut() {
            let mut j = *next / 2;
            while j < top {
                unsafe {
                    self.sieve.set_unchecked(j, true);
                }
                j += k;
            }

            // if this wraps, we've hit the limit, and so won't be
            // continuing, so whatever, it can be junk.
            *next = (2 * j + 1).wrapping_sub(SEG_LEN);
        }

        if low == 0 {
            // 1 is not prime.
            self.sieve.set(0, true);
        }

        Some((low, &self.sieve))
    }
}

#[cfg(test)]
mod tests {
    use super::StreamingSieve;
    #[test]
    fn test() {
        const LIMIT: usize = 2_000_000;
        let mut sieve = StreamingSieve::new(LIMIT);
        let primes = ::primal_smallsieve::Primes::sieve(LIMIT);
        while let Some((low, next)) = sieve.next() {
            let mut i = low + 1;
            while i < low + next.len() {
                if i > LIMIT { break }
                assert!(primes.is_prime(i) == !next[(i - low) / 2],
                        "failed for {} (is prime = {})", i, primes.is_prime(i));
                i += 2;
            }
        }
    }

}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    use test::Bencher;
    use super::StreamingSieve;

    fn run(b: &mut Bencher, n: usize) {
        b.iter(|| {
            let mut sieve = StreamingSieve::new(n);
            while sieve.next().is_some() {}
        })
    }

    #[bench]
    fn sieve_small(b: &mut Bencher) {
        run(b, 100)
    }
    #[bench]
    fn sieve_medium(b: &mut Bencher) {
        run(b, 10_000)
    }
    #[bench]
    fn sieve_large(b: &mut Bencher) {
        run(b, 100_000)
    }
    #[bench]
    fn sieve_larger(b: &mut Bencher) {
        run(b, 1_000_000)
    }
    #[bench]
    fn sieve_huge(b: &mut Bencher) {
        run(b, 10_000_000)
    }
}
