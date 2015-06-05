use primal_bit::{BitVec};
use std::{cmp};
use hamming;

use wheel;

/// A segmented sieve that yields only a small run of primes at a
/// time.
///
/// This is heavily inspired by this [segmented
/// sieve](http://primesieve.org/segmented_sieve.html) code.
#[derive(Debug)]
pub struct StreamingSieve {
    small: Option<::Sieve>,
    // stores which numbers *aren't* prime, i.e. true == composite.
    sieve: BitVec,
    primes: Vec<wheel::WheelInfo<wheel::Wheel210>>,
    small_primes: Vec<wheel::WheelInfo<wheel::Wheel30>>,
    presieve: BitVec,

    low: usize,
    current: usize,
    limit: usize,
}

const CACHE: usize = (32 << 10);
const SEG_ELEMS: usize = 8 * CACHE;
const SEG_LEN: usize = SEG_ELEMS * wheel::BYTE_MODULO / wheel::BYTE_SIZE;

const PRESIEVE_PROD: usize = 2 * 3 * 5 * 7 * 11 * 13;
const PRESIEVE_PRIMES: &'static [usize] = &[2, 3, 5, 7, 11, 13];
const PRESIEVE_NEXT: usize = 17;
const PRESIEVE_ACTIVE: bool = true;

fn bits_for(x: usize) -> usize {
    (x * wheel::BYTE_SIZE + wheel::BYTE_MODULO - 1) / wheel::BYTE_MODULO
}

#[inline(never)]
fn compute_presieve(limit_bits: usize) -> BitVec {
    assert!(PRESIEVE_ACTIVE);
    let len = cmp::min(bits_for(PRESIEVE_PROD),
                       limit_bits);
    let mut bitv = BitVec::from_elem(len, false);

    // this is silly and should be done with a sieve that only uses
    // the primes in PRESIEVE_PRIMES.
    for i in 0..len {
        let true_ = wheel::from_bit_index(i);
        fn gcd(x: usize, y: usize) -> usize {
            if y == 0 { x }
            else { gcd(y, x % y) }
        }
        if gcd(true_, PRESIEVE_PROD) != 1 {
            bitv.set(i, true)
        }
    }
    bitv
}

impl StreamingSieve {
    /// Create a new instance of the streaming sieve that will
    /// correctly progressively filter primes up to `limit`.
    pub fn new(limit: usize) -> StreamingSieve {
        let small = if limit < PRESIEVE_NEXT * PRESIEVE_NEXT {
            None
        } else {
            Some(::Sieve::new((limit as f64).sqrt() as usize + 1))
        };
        let current = PRESIEVE_NEXT;
        let low = 0;

        let elems = cmp::min(bits_for(limit), SEG_ELEMS);
        let presieve = compute_presieve(elems);

        StreamingSieve {
            small: small,
            sieve: BitVec::from_elem(elems, false),
            primes: vec![],
            small_primes: vec![],
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

    pub fn count_upto(n: usize) -> usize {
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
                    count += 8 * bytes.len() - hamming::weight(bytes) as usize;
                }
                let (tweak_byte, tweak_bit) = (tweak / 8, tweak % 8);
                let (_, last) = sieve.next().unwrap();
                let bytes = last.as_bytes();
                count += 8 * tweak_byte - hamming::weight(&bytes[..tweak_byte]) as usize;
                let byte = bytes[tweak_byte];
                for i in 0..tweak_bit + includes as usize {
                    count += (byte & (1 << i) == 0) as usize
                }
                count
            }
        }
    }

    pub fn very_small_sieve(&mut self, low: usize) {
        let offset = (low % PRESIEVE_PROD) * wheel::BYTE_SIZE / wheel::BYTE_MODULO / 8;

        copy_all(self.sieve.as_bytes_mut(),
                 self.presieve.as_bytes(),
                 offset);
        fn copy_all(mut dst: &mut [u8], src: &[u8], init_offset: usize) {
            let mut pos = 0;
            // pre-fill data at the start, as a rotation of `src`.
            pos += memcpy(&mut dst[pos..], &src[init_offset..]);
            if pos >= dst.len() { return }

            pos += memcpy(&mut dst[pos..], &src[..init_offset]);
            if pos >= dst.len() { return }

            // progressively copy the prefix to the rest of the
            // vector, doubling each time.
            while pos < dst.len() {
                let (filled, unfilled) = dst.split_at_mut(pos);
                pos += memcpy(unfilled, filled);
            }
        }
        fn memcpy<'d>(dst: &'d mut [u8], src: &[u8]) -> usize {
            use std::ptr;
            let l = cmp::min(dst.len(), src.len());
            unsafe {
                ptr::copy(src.as_ptr(), dst.as_mut_ptr(), l);
            }
            l
        }
    }

    fn small_primes_sieve(&mut self) {
        let bytes = self.sieve.as_bytes_mut();
        for wi in &mut self.small_primes {
            wi.sieve_hardcoded(bytes);
        }
    }

    fn direct_sieve(&mut self) {
        let bytes = self.sieve.as_bytes_mut();

        let mut iter = self.primes.iter_mut();

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
    pub fn next(&mut self) -> Option<(usize, &BitVec)> {
        if self.low >= self.limit {
            return None
        }

        let low = self.low;
        self.low += SEG_LEN;
        let high = cmp::min(low + SEG_LEN - 1, self.limit);
        self.sieve.clear();

        let mut s = self.current;

        if let Some(ref small) = self.small {
            while s * s <= high {
                if small.is_prime(s) {
                    if s <= SEG_LEN / 100 {
                        self.small_primes.push(wheel::compute_wheel_elem(wheel::Wheel30, s, low));
                    } else {
                        self.primes.push(wheel::compute_wheel_elem(wheel::Wheel210, s, low));
                    }
                }
                s += 1
            }
        }

        self.current = s;
        if PRESIEVE_ACTIVE && self.presieve.len() > 0 {
            self.very_small_sieve(low);
        }
        self.small_primes_sieve();
        self.direct_sieve();

        if low == 0 {
            // 1 is not prime.
            self.sieve.set(0, true);
            if PRESIEVE_ACTIVE {
                // but these are
                for &x in PRESIEVE_PRIMES {
                    let (use_, idx) = wheel::bit_index(x);
                    if use_ {
                        self.sieve.set(idx, false)
                    }
                }
            }
        }

        Some((low, &self.sieve))
    }
}

#[cfg(test)]
mod tests {
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
        let primes = ::primal_smallsieve::Primes::sieve(LIMIT);

        let mut base = 0;
        let mut index = 0;

        while let Some((_low, next)) = sieve.next() {
            for val in next {
                let i = wheel::BYTE_MODULO * base + coprime[index];
                if i >= LIMIT { break }
                assert!(primes.is_prime(i) == !val,
                        "failed for {} (is prime = {})", i, primes.is_prime(i));

                index += 1;
                if index == wheel::BYTE_SIZE {
                    index = 0;
                    base += 1
                }
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
    fn run_presieve(b: &mut Bencher, n: usize) {
        b.iter(|| super::compute_presieve(super::bits_for(n)))
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
    #[bench]
    fn presieve_small(b: &mut Bencher) {
        run_presieve(b, 100)
    }
    #[bench]
    fn presieve_medium(b: &mut Bencher) {
        run_presieve(b, 10_000)
    }
    #[bench]
    fn presieve_large(b: &mut Bencher) {
        run_presieve(b, 100_000)
    }
    #[bench]
    fn presieve_larger(b: &mut Bencher) {
        run_presieve(b, 1_000_000)
    }
    #[bench]
    fn presieve_huge(b: &mut Bencher) {
        run_presieve(b, 10_000_000)
    }
}
