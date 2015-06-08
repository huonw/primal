use wheel;
use streaming::{self, StreamingSieve};
use std::vec;
use std::collections::VecDeque;

const ITER_BASE_STEP: usize = 8 * wheel::BYTE_MODULO;

#[cfg(target_pointer_width = "32")]
const SQRT: usize = 1 << 16;
#[cfg(target_pointer_width = "64")]
const SQRT: usize = 1 << 32;
#[cfg(target_pointer_width = "32")]
type Queued = u16;
#[cfg(target_pointer_width = "64")]
type Queued = u32;

enum Early {
    Two,
    Three,
    Five,
    Done,
}

/// An iterator over all primes.
///
/// This will yield primes indefinitely (bits in `usize`
/// permitting). If there is an known upper bound, sieving first with
/// `Sieve` and using its `primes_from` method may be more efficient,
/// especially if the bound is small.
///
/// This requires *O(min(p, sqrt(X))* memory to yield prime `p`, where
/// `X` is the maximum value of `usize`.
///
/// # Examples
///
/// ```rust
/// # extern crate primal;
/// let count = primal::Primes::all().take_while(|p| *p < 1_000_000).count();
/// println!("{}", count);
/// ```
pub struct Primes {
    early: Early,
    base: usize,
    current: u64,
    elems: vec::IntoIter<u64>,
    streaming: StreamingSieve,
    queued: VecDeque<Queued>,
}

impl Primes {
    /// The sequence `2, 3, 5, 7, 11, ...`.
    pub fn all() -> Primes {
        let mut streaming = StreamingSieve::new(streaming::SEG_LEN);
        let mut iter = {
            let (_, bits) = streaming.next().unwrap();
            bits.as_u64s().to_owned().into_iter()
        };
        // we manually add the primes
        streaming.small = None;
        // go to the end.
        streaming.limit = !0;

        Primes {
            early: Early::Two,
            base: 0,
            current: iter.next().unwrap(),
            elems: iter,
            streaming: streaming,
            queued: VecDeque::new(),
        }
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self.early {
            Early::Done => {}
            Early::Two => {
                self.early = Early::Three;
                return Some(2)
            }
            Early::Three => {
                self.early = Early::Five;
                return Some(3)
            }
            Early::Five => {
                self.early = Early::Done;
                return Some(5)
            }
        }

        let mut c = self.current;
        'find_c: while c == 0 {
            for next in &mut self.elems {
                self.base += ITER_BASE_STEP;
                if next != 0 {
                    c = next;
                    break 'find_c
                }
            }
            let low = self.streaming.low;
            let high = low + streaming::SEG_LEN;
            while let Some(p) = self.queued.pop_front() {
                let q = p as usize;
                if q * q < high {
                    self.streaming.add_sieving_prime(q, low)
                } else {
                    self.queued.push_front(p);
                    break
                }
            }
            match self.streaming.next() {
                Some((_, bits)) => self.elems = bits.as_u64s().to_owned().into_iter(),
                None => return None,
            }
        }

        let lsb = c.trailing_zeros();
        self.current = c & (c - 1);
        let p = self.base + wheel::TRUE_AT_BIT_64[lsb as usize];
        if ((streaming::SEG_LEN as f64).sqrt() as usize) < p && p < SQRT {
            self.queued.push_back(p as Queued);
        }
        Some(p)
    }
}

#[cfg(test)]
mod tests {
    use Sieve;
    use super::Primes;

    #[test]
    fn equality() {
        let limit = 2_000_000;
        let sieve = Sieve::new(limit);

        let real = sieve.primes_from(0).take_while(|x| *x < limit);
        let computed = Primes::all().take_while(|x| *x < limit);

        let mut i = 0;
        for (r, c) in real.zip(computed) {
            assert_eq!(c, r);
            i += 1;
        }
        assert_eq!(sieve.count_upto(limit), i);
    }
}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    use super::Primes;
    use test::Bencher;
    fn bench_iterate(b: &mut Bencher, upto: usize) {
        b.iter(|| {
            Primes::all().take_while(|x| *x <= upto).count()
        })
    }

    #[bench]
    fn iterate_small(b: &mut Bencher) { bench_iterate(b, 100) }
    #[bench]
    fn iterate_large(b: &mut Bencher) { bench_iterate(b, 100_000) }
    #[bench]
    fn iterate_huge(b: &mut Bencher) { bench_iterate(b, 10_000_000) }
}
