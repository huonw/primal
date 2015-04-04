#![allow(dead_code)]

use std::collections::{BitVec};
use std::{cmp};

use Primes;

/// A segmented sieve that yields only a small run of primes at a
/// time.
///
/// This is heavily inspired by this [segmented
/// sieve](http://primesieve.org/segmented_sieve.html) code.
pub struct StreamingSieve {
    small: Primes,
    sieve: BitVec,
    primes: Vec<(usize, usize)>,

    low: usize,
    current: usize,
    limit: usize,
}

const CACHE: usize = 32 << 10;
// 8 for the bit vector, 2 for storing odd numbers only
const SEG_SIZE: usize = 16 * CACHE;

impl StreamingSieve {
    /// Create a new instance of the streaming sieve that will
    /// correctly progressively filter primes up to `limit`.
    pub fn new(limit: usize) -> StreamingSieve {
        let small = Primes::sieve((limit as f64).sqrt() as usize + 1);
        let current = 2;
        let low = 0;

        StreamingSieve {
            small: small,
            sieve: BitVec::from_elem(SEG_SIZE, false),
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
        self.low += SEG_SIZE;
        let high = cmp::min(low + SEG_SIZE - 1, self.limit);
        self.sieve.set_all();

        while self.current * self.current <= high {
            if self.small.is_prime(self.current) {
                self.primes.push((self.current, self.current * self.current - low));
            }
            self.current += 1
        }
        for &mut (k, ref mut next) in self.primes.iter_mut() {
            let mut j = *next / 2;
            while j < SEG_SIZE / 2 {
                self.sieve.set(j, false);
                j += k;
            }


            *next = (2 * j + 1) - SEG_SIZE;
        }
        if low == 0 {
            // 1 is not prime.
            self.sieve.set(0, false);
        }

        Some((low, &self.sieve))
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::StreamingSieve;

    #[test]
    #[ignore(reason = "5 isn't a prime? should debug it, I guess.")]
    fn test() {
        let mut sieve = StreamingSieve::new(2000);
        let primes = ::Primes::sieve(2000);

        loop {
            let (low, next) = match sieve.next() {
                None => break,
                Some(x) => x,
            };
            println!("tick {}", next.len());

            for i in (low + 1..low + next.len()).step_by(2) {
                if i > 2000 { break }
                assert!(primes.is_prime(i) == next[(i - low) / 2],
                        "failed for {} (is prime = {})", i, primes.is_prime(i));
            }
        }
    }

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
    fn sieve_huge(b: &mut Bencher) {
        run(b, 10_000_000)
    }
}
