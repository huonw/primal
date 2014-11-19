#![allow(dead_code)]
// inspired by http://primesieve.org/segmented_sieve.html

use std::collections::{Bitv};
use std::{cmp};
use std::num::Float;

use Primes;

pub struct Sieve {
    small: Primes,
    sieve: Bitv,
    primes: Vec<uint>,
    next: Vec<uint>,

    low: uint,
    current: uint,
    limit: uint,
}

const CACHE: uint = 32 << 10;
// 8 for the bit vector, 2 for storing odd numbers only
const SEG_SIZE: uint = 16 * CACHE;

impl Sieve {
    pub fn new(limit: uint) -> Sieve {
        let small = Primes::sieve((limit as f64).sqrt() as uint + 1);
        let current = 2;
        let low = 0;

        Sieve {
            small: small,
            sieve: Bitv::with_capacity(SEG_SIZE, false),
            primes: vec![],
            next: vec![],

            low: low,
            current: current,
            limit: limit
        }
    }

    pub fn next(&mut self) -> Option<(uint, &Bitv)> {
        if self.low >= self.limit {
            return None
        }

        let low = self.low;
        self.low += SEG_SIZE;
        let high = cmp::min(low + SEG_SIZE - 1, self.limit);
        self.sieve.set_all();

        while self.current * self.current <= high {
            if self.small.is_prime(self.current) {
                self.primes.push(self.current);
                self.next.push(self.current * self.current - low);
            }
            self.current += 1
        }
        for i in range(1, self.primes.len()) {
            let mut j = self.next[i] / 2;
            let k = self.primes[i];
            while j < SEG_SIZE / 2 {
                self.sieve.set(j, false);
                j += k;
            }


            self.next[i] = (2 * j + 1) - SEG_SIZE;
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
    use super::Sieve;
    use std::iter::range_step;

    #[test]
    fn test() {
        let mut sieve = Sieve::new(2000);
        let primes = ::Primes::sieve(2000);

        loop {
            let (low, next) = match sieve.next() {
                None => break,
                Some(x) => x,
            };
            println!("tick {}", next.len());

            for i in range_step(low + 1, low + next.len(), 2) {
                if i > 2000 { break }
                assert!(primes.is_prime(i) == next[(i - low) / 2],
                        "failed for {} (is prime = {})", i, primes.is_prime(i));
            }
        }
    }

    fn run(b: &mut Bencher, n: uint) {
        b.iter(|| {
            let mut sieve = Sieve::new(n);
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
