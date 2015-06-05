use primal_bit::BitVec;
use std::{cmp, ptr};

use wheel;
use super::{bits_for, StreamingSieve};

const PRESIEVE_PROD: usize = 2 * 3 * 5 * 7 * 11 * 13;
const PRESIEVE_PRIMES: &'static [usize] = &[2, 3, 5, 7, 11, 13];
pub const PRESIEVE_NEXT: usize = 17;

#[derive(Debug)]
pub struct Presieve {
    sieve: BitVec,
}
impl Presieve {
    pub fn new(limit_bits: usize) -> Presieve {
        let len = cmp::min(bits_for(PRESIEVE_PROD),
                           limit_bits);

        let mut sievers = vec![];
        for &x in PRESIEVE_PRIMES {
            let (use_, _idx) = wheel::bit_index(x);
            if use_ {
                sievers.push(wheel::compute_wheel_elem(wheel::Wheel30, x, PRESIEVE_PROD));
            }
        }
        let mut sieve =  BitVec::from_elem(len, false);
        StreamingSieve::small_primes_sieve(&mut sieve, &mut sievers);
        Presieve {
            sieve: sieve,
        }
    }
    pub fn mark_small_primes(&self, sieve: &mut BitVec) {
        for &x in PRESIEVE_PRIMES {
            let (use_, idx) = wheel::bit_index(x);
            if use_ {
                sieve.set(idx, false)
            }
        }
    }
    pub fn apply(&self, sieve: &mut BitVec, low: usize) {
        let offset = (low % PRESIEVE_PROD) * wheel::BYTE_SIZE / wheel::BYTE_MODULO / 8;

        copy_all(sieve.as_bytes_mut(),
                 self.sieve.as_bytes(),
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
            let l = cmp::min(dst.len(), src.len());
            unsafe {
                ptr::copy(src.as_ptr(), dst.as_mut_ptr(), l);
            }
            l
        }
    }
}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    use test::Bencher;
    use super::Presieve;
    fn run_presieve(b: &mut Bencher, n: usize) {
        b.iter(|| super::Presieve::new(super::super::bits_for(n)))
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
