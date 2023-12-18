use primal_bit::BitVec;
use core::cmp;

#[cfg(feature = "no-std")]
use alloc::{vec, vec::Vec};

use crate::wheel;

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
/// let count = primal::StreamingSieve::prime_pi(123456);
/// println!("𝜋(123456) = {}", count);
/// ```
#[derive(Debug)]
pub struct StreamingSieve {
    small: Option<crate::Sieve>,
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
    pub(crate) fn new(limit: usize) -> StreamingSieve {
        let low = 0;

        let elems = cmp::min(wheel::bits_for(limit), SEG_ELEMS);
        let presieve = presieve::Presieve::new(elems);
        let current = presieve.smallest_unincluded_prime();

        let small = if limit < current * current {
            None
        } else {
            Some(crate::Sieve::new(isqrt(limit) + 1))
        };

        StreamingSieve {
            small,
            sieve: BitVec::from_elem(elems, true),
            primes: vec![],
            small_primes: vec![],
            large_primes: vec![],
            presieve,

            low,
            current,
            limit
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
    /// assert_eq!(primal::StreamingSieve::prime_pi(10), 4);
    /// // the endpoint is included
    /// assert_eq!(primal::StreamingSieve::prime_pi(11), 5);
    ///
    /// assert_eq!(primal::StreamingSieve::prime_pi(100), 25);
    /// assert_eq!(primal::StreamingSieve::prime_pi(1000), 168);
    /// ```
    pub fn prime_pi(n: usize) -> usize {
        match n {
            0..=1 => 0,
            2 => 1,
            3..=4 => 2,
            5..=6 => 3,
            7..=10 => 4,
            _ => {
                let mut sieve = StreamingSieve::new(n);
                let (includes, base, tweak) = sieve.index_for(n);
                let mut count = match wheel::BYTE_MODULO {
                    30 => 3,
                    _ => unimplemented!()
                };

                for _ in 0..base {
                    let (_, bitv) = sieve.next().unwrap();
                    count += bitv.count_ones();
                }

                let tail = tweak + includes as usize;
                if tail != 0 {
                    let (_, last) = sieve.next().unwrap();
                    count += last.count_ones_before(tail);
                }
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
                    let count = bits.count_ones();
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
        if p <= CACHE / 2 {
            self.small_primes.push(wheel::State::new(wheel::Wheel30, p, low));
        } else {
            let elem = wheel::State::new(wheel::Wheel210, p, low);
            if p < CACHE * 5 / 2 {
                self.primes.push(elem)
            } else {
                self.large_primes.push(elem)
            }
        }
    }

    fn find_new_sieving_primes(&mut self, low: usize, high: usize) {
        if let Some(small) = self.small.take() {
            for p in small.primes_from(self.current) {
                if p * p > high {
                    self.current = p;
                    break
                }
                self.add_sieving_prime(p, low);
            }
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
        let mut chunks = self.primes.chunks_exact_mut(3);

        while let Some([wi1, wi2, wi3]) = chunks.next() {
            wi1.sieve_triple(wi2, wi3, bytes);
        }

        for wi in chunks.into_remainder() {
            wi.sieve(bytes);
        }
    }

    fn large_primes_sieve(&mut self) {
        let bytes = self.sieve.as_bytes_mut();
        let mut chunks = self.large_primes.chunks_exact_mut(2);

        while let Some([wi1, wi2]) = chunks.next() {
            wi1.sieve_pair(wi2, bytes);
        }

        for wi in chunks.into_remainder() {
            wi.sieve(bytes);
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
    pub(crate) fn next(&mut self) -> Option<(usize, &BitVec)> {
        if self.low >= self.limit {
            return None
        }

        let low = self.low;
        self.low = self.low.saturating_add(SEG_LEN);
        let high = cmp::min(low.saturating_add(SEG_LEN - 1), self.limit);

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

#[cfg(test)]
mod tests {
    use crate::Sieve;
    use primal_slowsieve::Primes;
    use crate::wheel;
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
        let (limit, mult) = if cfg!(feature = "slow_tests") {
            (2_000_000, 19_998)
        } else {
            (200_000, 1_998)
        };
        let real = Primes::sieve(limit);

        for i in (0..20).chain((0..100).map(|n| n * mult + 1)) {
            let val = StreamingSieve::prime_pi(i);
            let true_ = real.primes().take_while(|p| *p <= i).count();
            assert!(val == true_, "failed for {}, true {}, computed {}",
                    i, true_, val)
        }
    }

    #[test]
    fn prime_pi_issue_48() {
        // Expected values independently confirmed on Wolfram|Alpha.
        let limits = [49, 121, 289, 961, 11_047, 32_611, 230_907, 455_166_135];
        let expected = [15, 30, 61, 162, 1_338, 3_501, 20_513, 24_112_077];
        for (&limit, &expected) in limits.iter().zip(&expected) {
            let val = StreamingSieve::prime_pi(limit);
            assert_eq!(val, expected, "failed for limit {}", limit);
        }
    }

    #[test]
    fn prime_pi_seg_len() {
        const PRIME1: usize = 982_981;
        const PRIME2: usize = 983_063;
        const EXPECTED: usize = 77_279;
        assert!((PRIME1..PRIME2).contains(&super::SEG_LEN));
        assert_eq!(StreamingSieve::prime_pi(PRIME1 - 1), EXPECTED - 1);
        for limit in PRIME1..PRIME2 {
            assert_eq!(StreamingSieve::prime_pi(limit), EXPECTED);
        }
        assert_eq!(StreamingSieve::prime_pi(PRIME2), EXPECTED + 1);
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

    // These are designed to specifically test the medium sized and
    // large prime sieving.
    #[test]
    fn prime_pi_huge() {
        #[cfg(all(feature = "slow_tests", target_pointer_width = "64"))]
        const LIMIT_RESULT: (usize, usize) = (10_000_000_000, 455_052_511);
        #[cfg(all(feature = "slow_tests", target_pointer_width = "32"))]
        const LIMIT_RESULT: (usize, usize) = (4_294_000_000, 203_236_859);
        #[cfg(not(feature = "slow_tests"))]
        const LIMIT_RESULT: (usize, usize) = (500_000_000, 26_355_867);

        assert_eq!(StreamingSieve::prime_pi(LIMIT_RESULT.0), LIMIT_RESULT.1);
    }


    #[test]
    fn nth_prime_huge() {
        #[cfg(all(feature = "slow_tests", target_pointer_width = "64"))]
        const LIMIT_RESULT: (usize, usize) = (455_052_512, 10_000_000_019);
        #[cfg(all(feature = "slow_tests", target_pointer_width = "32"))]
        const LIMIT_RESULT: (usize, usize) = (203_236_860, 4_294_000_079);
        #[cfg(not(feature = "slow_tests"))]
        const LIMIT_RESULT: (usize, usize) = (26_355_868, 500_000_003);

        assert_eq!(StreamingSieve::nth_prime(LIMIT_RESULT.0), LIMIT_RESULT.1);
    }
}
