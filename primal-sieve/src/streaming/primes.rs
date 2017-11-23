use wheel;
use streaming::{self, StreamingSieve};
use std::vec;

const ITER_BASE_STEP: usize = 8 * wheel::BYTE_MODULO;

#[cfg(target_pointer_width = "32")]
const SQRT: usize = 1 << 16;
#[cfg(target_pointer_width = "64")]
const SQRT: usize = 1 << 32;

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
/// This requires *O(sqrt(p))* memory to yield prime `p`, where `X` is
/// the maximum value of `usize`.
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
    sieving_primes: Option<Box<Primes>>,
    left_over: Option<usize>,
}

impl Primes {
    /// The sequence `2, 3, 5, 7, 11, ...`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate primal;
    /// // print the first 20 primes
    /// for p in primal::Primes::all().take(20) {
    ///     println!("{}", p);
    /// }
    /// ```
    pub fn all() -> Primes {
        Primes::sqrt(SQRT)
    }

    fn sqrt(sqrt: usize) -> Primes {
        let (sieving, limit) = if sqrt < streaming::isqrt(streaming::SEG_LEN) {
            (None, sqrt * sqrt)
        } else {
            (Some(Box::new(Primes::sqrt(streaming::isqrt(sqrt)))),
             streaming::SEG_LEN)
        };
        let mut streaming = StreamingSieve::new(limit);

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
            sieving_primes: sieving,
            left_over: None,
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

            for q in self.left_over.into_iter().chain(self.sieving_primes.as_mut().unwrap()) {
                if q * q > high {
                    self.left_over = Some(q);
                    break
                }
                if q > streaming::isqrt(streaming::SEG_LEN) {
                    self.streaming.add_sieving_prime(q, low);
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
        Some(p)
    }
}

#[cfg(test)]
mod tests {
    use Sieve;
    use super::Primes;

    #[test]
    fn equality() {
        let limit = 20_000_000;
        let sieve = Sieve::new(limit);

        let real = sieve.primes_from(0).take_while(|x| *x < limit);
        let computed = Primes::all().take_while(|x| *x < limit);

        let mut i = 0;
        for (r, c) in real.zip(computed) {
            assert_eq!(c, r);
            i += 1;
        }
        assert_eq!(sieve.prime_pi(limit), i);
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
