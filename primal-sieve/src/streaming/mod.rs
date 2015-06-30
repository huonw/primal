use primal_estimate;
use primal_bit::{BitVec};
use std::{cmp};
use hamming;

use wheel;

pub mod primes;
mod presieve;

/// A heavily optimised prime sieve.
///
/// This is a streaming segmented sieve, meaning it sieves numbers in
/// intervals, extracting whatever it needs and discarding it. See
/// `Sieve` for a wrapper that caches the information to allow for
/// repeated queries, at the cost of *O(limit)* memory use.
///
/// This uses *O(sqrt(limit))* memory, and is designed to be as
/// cache-friendly as possible. `StreamingSieve` should be used for
/// one-off calls, or simple linear iteration.
///
/// The design is *heavily* inspired/adopted from Kim Walisch's
/// [primesieve](http://primesieve.org/), and has similar speed
/// (around 5-20% slower).
///
/// # Examples
///
/// ```rust
/// # extern crate primal;
/// let count = primal::StreamingSieve::prime_pi(123456);
/// println!("𝜋(123456) = {}", count);
/// ```
#[derive(Debug)]
pub struct StreamingSieve {
    small: Option<::Sieve>,
    sieve: BitVec,
    primes: Vec<wheel::State<wheel::Wheel210>>,
    small_primes: Vec<wheel::State<wheel::Wheel30>>,
    large_primes: Vec<wheel::State<wheel::Wheel210>>,
    presieve: presieve::Presieve,

    low: usize,
    current: usize,
    limit: usize,
}

const CACHE: usize = 32 << 10;
const SEG_ELEMS: usize = 8 * CACHE;
const SEG_LEN: usize = SEG_ELEMS * wheel::BYTE_MODULO / wheel::BYTE_SIZE;

fn isqrt(x: usize) -> usize {
    (x as f64).sqrt() as usize
}

impl StreamingSieve {
    /// Create a new instance of the streaming sieve that will
    /// correctly progressively filter primes up to `limit`.
    fn new(limit: usize) -> StreamingSieve {
        let low = 0;

        let elems = cmp::min(wheel::bits_for(limit), SEG_ELEMS);
        let presieve = presieve::Presieve::new(elems);
        let current = presieve.smallest_unincluded_prime();

        let small = if limit < current * current {
            None
        } else {
            Some(::Sieve::new(isqrt(limit) + 1))
        };

        StreamingSieve {
            small: small,
            sieve: BitVec::from_elem(elems, true),
            primes: vec![],
            small_primes: vec![],
            large_primes: vec![],
            presieve: presieve,

            low: low,
            current: current,
            limit: limit
        }
    }
    fn split_index(&self, idx: usize) -> (usize, usize) {
        let len = SEG_ELEMS;
        (idx / len,idx % len)
    }
    fn index_for(&self, n: usize) -> (bool, usize, usize) {
        let (b, idx) = wheel::bit_index(n);
        let (base, tweak) = self.split_index(idx);
        (b, base, tweak)
    }

    /// Count the number of primes upto and including `n`, that is, 𝜋,
    /// the [prime counting
    /// function](https://en.wikipedia.org/wiki/Prime-counting_function).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate primal;
    /// assert_eq!(primal::StreamingSieve::prime_pi(10), 4);
    /// // the endpoint is included
    /// assert_eq!(primal::StreamingSieve::prime_pi(11), 5);
    ///
    /// assert_eq!(primal::StreamingSieve::prime_pi(100), 25);
    /// assert_eq!(primal::StreamingSieve::prime_pi(1000), 168);
    /// ```
    pub fn prime_pi(n: usize) -> usize {
        match n {
            0...1 => 0,
            2 => 1,
            3...4 => 2,
            5...6 => 3,
            7...10 => 4,
            _ => {
                let mut sieve = StreamingSieve::new(n);
                let (includes, base, tweak) = sieve.index_for(n);
                let mut count = match wheel::BYTE_MODULO {
                    30 => 3,
                    _ => unimplemented!()
                };

                for _ in 0..base {
                    let (_, bitv) = sieve.next().unwrap();
                    let bytes = bitv.as_bytes();
                    count += hamming::weight(bytes) as usize;
                }

                let (_, last) = sieve.next().unwrap();
                count += last.count_ones_before(tweak + includes as usize);
                count
            }
        }
    }

    /// Compute *p<sub>n</sub>*, the `n` prime number, 1-indexed
    /// (i.e. *p<sub>1</sub>* = 2, *p<sub>2</sub>* = 3).
    ///
    /// # Panics
    ///
    /// `n` must be larger than 0 and less than the total number of
    /// primes in this sieve (that is,
    /// `self.prime_pi(self.upper_bound())`).
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate primal;
    /// assert_eq!(primal::StreamingSieve::nth_prime(1_000), 7919);
    /// ```
    pub fn nth_prime(n: usize) -> usize {
        assert!(n > 0);
        match n {
            1 => 2,
            2 => 3,
            3 => 5,
            _ => {
                let mut bit_n = n - 3;
                let (_, hi) = primal_estimate::nth_prime(n as u64);
                let mut sieve = StreamingSieve::new(hi as usize);
                while let Some((low, bits)) = sieve.next() {
                    let count = hamming::weight(bits.as_bytes()) as usize;
                    if count >= bit_n {
                        let bit_idx = bits.find_nth_bit(bit_n - 1).unwrap();
                        return low + wheel::from_bit_index(bit_idx)
                    }

                    bit_n -= count
                }
                unreachable!()
            }
        }
    }

    fn add_sieving_prime(&mut self, p: usize, low: usize) {
        if p <= SEG_LEN / 100 {
            self.small_primes.push(wheel::compute_wheel_elem(wheel::Wheel30, p, low));
        } else {
            let elem = wheel::compute_wheel_elem(wheel::Wheel210, p, low);
            if p < SEG_LEN / 2 {
                self.primes.push(elem)
            } else {
                self.large_primes.push(elem)
            }
        }
    }

    fn find_new_sieving_primes(&mut self, low: usize, high: usize) {
        if let Some(small) = self.small.take() {
            let mut s = self.current;
            assert!(s % 2 == 1);
            while s * s <= high {
                if small.is_prime(s) {
                    self.add_sieving_prime(s, low)
                }
                s += 2
            }

            self.current = s;
            self.small = Some(small);
        }
    }

    fn small_primes_sieve<W: wheel::Wheel>(sieve: &mut BitVec,
                                           small_primes: &mut [wheel::State<W>]) {
        let bytes = sieve.as_bytes_mut();
        for wi in small_primes {
            wi.sieve_hardcoded(bytes);
        }
    }

    fn direct_sieve(&mut self) {
        let bytes = self.sieve.as_bytes_mut();

        let mut iter = self.primes.iter_mut();

        while iter.size_hint().0 >= 3 {
            match (iter.next(), iter.next(), iter.next()) {
                (Some(wi1), Some(wi2), Some(wi3)) => {
                    wi1.sieve_triple(wi2, wi3, bytes);
                }
                _ => unreachable!()
            }
        }
        for wi in iter {
            wi.sieve(bytes)
        }
    }

    fn large_primes_sieve(&mut self) {
        let bytes = self.sieve.as_bytes_mut();

        let mut iter = self.large_primes.iter_mut();

        while iter.size_hint().0 >= 2 {
            match (iter.next(), iter.next()) {
                (Some(wi1), Some(wi2)) => {
                    wi1.sieve_pair(wi2, bytes);
                }
                _ => unreachable!()
            }
        }
        for wi in iter {
            wi.sieve(bytes)
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
    fn next(&mut self) -> Option<(usize, &BitVec)> {
        if self.low >= self.limit {
            return None
        }

        let low = self.low;
        self.low += SEG_LEN;
        let high = cmp::min(low + SEG_LEN - 1, self.limit);

        self.find_new_sieving_primes(low, high);

        self.presieve.apply(&mut self.sieve, low);
        StreamingSieve::small_primes_sieve(&mut self.sieve, &mut self.small_primes);
        self.direct_sieve();
        self.large_primes_sieve();

        if low == 0 {
            // 1 is not prime.
            self.sieve.set(0, false);
            self.presieve.mark_small_primes(&mut self.sieve);
        }

        Some((low, &self.sieve))
    }
}

// module-public but crate-private wrappers, to allow `Sieve` to call these functions.
pub fn new(limit: usize) -> StreamingSieve {
    StreamingSieve::new(limit)
}
pub fn next(sieve: &mut StreamingSieve) -> Option<(usize, &BitVec)> {
    sieve.next()
}

#[cfg(test)]
mod tests {
    use Sieve;
    use primal_slowsieve::Primes;
    use wheel;
    use super::StreamingSieve;
    fn gcd(x: usize, y: usize) -> usize {
        if y == 0 { x }
        else { gcd(y, x % y) }
    }
    fn coprime_to(x: usize) -> Vec<usize> {
        (1..x).filter(|&n| gcd(n, x) == 1).collect()
    }
    #[test]
    fn test() {
        let coprime = coprime_to(wheel::BYTE_MODULO);
        const LIMIT: usize = 2_000_000;
        let mut sieve = StreamingSieve::new(LIMIT);
        let primes = ::primal_slowsieve::Primes::sieve(LIMIT);

        let mut base = 0;
        let mut index = 0;

        while let Some((_low, next)) = sieve.next() {
            for val in next {
                let i = wheel::BYTE_MODULO * base + coprime[index];
                if i >= LIMIT { break }
                assert!(primes.is_prime(i) == val,
                        "failed for {} (is prime = {})", i, primes.is_prime(i));

                index += 1;
                if index == wheel::BYTE_SIZE {
                    index = 0;
                    base += 1
                }
            }
        }
    }
    #[test]
    fn prime_pi() {
        let limit = 2_000_000;
        let real = Primes::sieve(limit);

        for i in (0..20).chain((0..100).map(|n| n * 19998 + 1)) {
            let val = StreamingSieve::prime_pi(i);
            let true_ = real.primes().take_while(|p| *p <= i).count();
            assert!(val == true_, "failed for {}, true {}, computed {}",
                    i, true_, val)
        }
    }

    #[test]
    fn nth_prime() {
        let primes = Sieve::new(2_000_000);

        for (i, p) in primes.primes_from(0).enumerate() {
            let n = i + 1;
            if n < 2000 || n % 1000 == 0 {
                assert_eq!(StreamingSieve::nth_prime(n), p);
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
