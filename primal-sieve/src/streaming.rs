use primal_bit::{BitVec};
use std::{cmp};

use wheel;
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
    primes: Vec<wheel::WheelInfo>,
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
        let small = Primes::sieve((limit as f64).sqrt() as usize + 1);
        let mut current = match wheel::MODULO {
            6 => 5,
            30 => 7,
            210 => 11,
            _ => unimplemented!(),
        };
        let low = 0;

        let elems = cmp::min(bits_for(limit), SEG_ELEMS);
        let presieve = if PRESIEVE_ACTIVE {
            current = PRESIEVE_NEXT;
            compute_presieve(elems)
        } else {
            BitVec::new()
        };

        StreamingSieve {
            small: small,
            sieve: BitVec::from_elem(elems, false),
            primes: vec![],
            presieve: presieve,

            low: low,
            current: current,
            limit: limit
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

    fn direct_sieve(&mut self) {
        let bytes = self.sieve.as_bytes_mut();
        let top = bytes.len();

        let mut iter = self.primes.iter_mut();

        while iter.size_hint().0 >= 2 {
            match (iter.next(), iter.next()) {
                (Some(wi1), Some(wi2)) => {
                    let mut si1 = wi1.sieve_index;
                    let mut wi_1 = wi1.wheel_index;
                    let p1 = wi1.prime;
                    let mut si2 = wi2.sieve_index;
                    let mut wi_2 = wi2.wheel_index;
                    let p2 = wi2.prime;

                    while si1 < top && si2 < top {
                        wheel::set_bit(bytes, &mut si1, &mut wi_1, p1);
                        wheel::set_bit(bytes, &mut si2, &mut wi_2, p2);
                    }
                    while si1 < top {
                        wheel::set_bit(bytes, &mut si1, &mut wi_1, p1);
                    }
                    while si2 < top {
                        wheel::set_bit(bytes, &mut si2, &mut wi_2, p2);
                    }

                    // if this wraps, we've hit the limit, and so won't be
                    // continuing, so whatever, it can be junk.
                    wi1.sieve_index = si1.wrapping_sub(top);
                    wi1.wheel_index = wi_1;
                    wi2.sieve_index = si2.wrapping_sub(top);
                    wi2.wheel_index = wi_2;
                }
                _ => unreachable!()
            }
        }
        for wi in iter {
            let mut si = wi.sieve_index;
            let mut wi_ = wi.wheel_index;
            let p = wi.prime;
            while si < top {
                wheel::set_bit(bytes, &mut si, &mut wi_, p);
            }
            // if this wraps, we've hit the limit, and so won't be
            // continuing, so whatever, it can be junk.
            wi.sieve_index = si.wrapping_sub(top);
            wi.wheel_index = wi_;
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
                self.primes.push(wheel::compute_wheel_elem(s, low));
            }
            s += 1
        }

        self.current = s;
        if PRESIEVE_ACTIVE && self.presieve.len() > 0 {
            self.very_small_sieve(low);
        }
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
