//! Estimate upper and lower bounds for the *n*-th prime, and π(*n*),
//! the number of primes less than or equal to *n*.
//!
//! This is designed to be used via the `primal` crate.

#![deny(warnings)]

#[allow(dead_code)]
mod tables;

/// Returns estimated bounds for π(*n*), the number of primes less
/// than or equal to `n`.
///
/// That is, if (*a*, *b*) = `estimate_prime_pi(n)`, *a* ≤ π(*n*) ≤
/// *b*. The bounds used are proved in \[1], \[2, Théorème 1.10]
/// (both summarised in \[2, pp. 14–15]), and \[3, Section 6.2].
///
/// \[1]: Barkley Rosser. "Explicit Bounds for Some Functions of Prime
/// Numbers". American Journal of Mathematics 63 (1):
/// 211–232. 1941. doi:[10.2307/2371291](http://dx.doi.org/10.2307/2371291).
///
/// \[2]: Dusart, Pierre. ["Autour de la fonction qui compte le nombre
/// de nombres premiers."][pdf] PhD diss., Université de Limoges,
/// 1998.
///
/// [pdf]: http://www.unilim.fr/laco/theses/1998/T1998_01.html
///
/// \[3]: Dusart, Pierre. "Estimates of Some Functions Over Primes
/// without R.H."
/// ArXiv:[1002.0442](http://arxiv.org/abs/1002.0442). 2010.
///
/// # Examples
///
/// ```rust
/// # extern crate primal;
/// // the number of primes below 1e9
/// let count_billion = 50_847_534;
///
/// let (low, high) = primal::estimate_prime_pi(1_000_000_000);
/// // check the bounds are satisfied
/// assert!(low <= count_billion && count_billion <= high);
/// ```
pub fn prime_pi(n: u64) -> (u64, u64) {
    if n < tables::SMALL_PRIME_PI.len() as u64 {
        let x = tables::SMALL_PRIME_PI[n as usize] as u64;
        (x, x)
    } else {
        let n_ = n as f64;
        let lg = n_.ln();
        let inv_lg = 1.0 / lg;
        let n_inv_lg = n_ * inv_lg;

        let lo = match () {
            // [3] Theorem 6.9 (6.7)
            _ if n >= 88783_u64 => n_inv_lg * (1.0 + inv_lg * (1.0 + 2.0 * inv_lg)),
            // [2] Theorem 1.10 (6.)
            _ if n >= 32299_u64 => n_inv_lg * (1.0 + inv_lg * (1.0 + 1.8 * inv_lg)),
            // [2] Theorem 1.10 (5.)
            _ if n >= 5393_u64 => n_ / (lg - 1.0),
            // [2] Theorem 1.10 (1.)
            _ if n >= 599_u64 => n_inv_lg * (1.0 + inv_lg),
            // [1]
            _ => n_ / (lg + 2.0),
        };

        let hi = match () {
            // [3] Theorem 6.9 (6.7)
            _ if n >= 16537307828_u64 => n_inv_lg * (1.0 + inv_lg * (1.0 + 2.334 * inv_lg)),
            // [2] Theorem 1.10 (3.)
            _ if n >= 13220000000_u64 => n_inv_lg * (1.0 + 1.0992 * inv_lg),
            // [3] Theorem 6.9 (6.7)
            _ if n >= 2953652287_u64 => n_inv_lg * (1.0 + inv_lg * (1.0 + 2.334 * inv_lg)),
            // [2] Theorem 1.10 (3.)
            _ if n >= 355991_u64 => n_inv_lg * (1.0 + inv_lg * (1.0 + 2.51 * inv_lg)),
            // [2] Theorem 1.10 (4.)
            _ if n >= 60184_u64 => n_ / (lg - 1.1),
            // [2] Theorem 1.10 (2.)
            _ => n_inv_lg * (1.0 + 1.2762 * inv_lg),
        };

        (lo as u64, hi as u64)
    }
}

/// Gives estimated bounds for *p<sub>n</sub>*, the `n`th prime number,
/// 1-indexed (i.e. *p<sub>1</sub>* = 2, *p<sub>2</sub>* = 3).
///
/// That is, if (<i>a</i>,<i>b</i>) = `estimate_nth_prime(n)`, *a* ≤
/// *p<sub>n</sub>* ≤ *b*. The bounds used are proved in \[1], \[2,
/// Théorèmes 1.6–1.8] (both summarised in \[2, pp. 14–15]) and \[3,
/// Section 6.1.2].
///
/// \[1]: Massias, Jean-Pierre; Robin, Guy. ["Bornes effectives pour
/// certaines fonctions concernant les nombres
/// premiers."](http://eudml.org/doc/247826) Journal de théorie des
/// nombres de Bordeaux 8.1 (1996): 215-242.
///
/// \[2]: Dusart, Pierre. ["Autour de la fonction qui compte le nombre
/// de nombres premiers."][pdf] PhD diss., Université de Limoges, 1998.
///
/// [pdf]: http://www.unilim.fr/laco/theses/1998/T1998_01.html
///
/// \[3]: Dusart, Pierre. "Estimates of Some Functions Over Primes
/// without R.H."
/// ArXiv:[1002.0442](http://arxiv.org/abs/1002.0442). 2010.
///
/// # Examples
///
/// ```rust
/// # extern crate primal;
/// // the 1e9-th prime
/// let billionth = 22_801_763_489;
///
/// let (low, high) = primal::estimate_nth_prime(1_000_000_000);
/// // check the bounds are satisfied
/// assert!(low <= billionth && billionth <= high);
/// ```
pub fn nth_prime(n: u64) -> (u64, u64) {
    const MAX_VALID_INPUT: u64 = 425281711831682432;
    assert!(n <= MAX_VALID_INPUT, "nth_prime({}) overflows a u64", n);

    if n == 0 {
        (0, 0)
    } else if n <= tables::SMALL_PRIMES.len() as u64 {
        // table is 0-indexed, n is 1-indexed, need to adjust.
        let x = tables::SMALL_PRIMES[n as usize - 1] as u64;
        (x, x)
    } else {
        let n_ = n as f64;
        let lg = n_.ln();
        let lglg = lg.ln();

        let lo = match () {
            // [2] Theorem 1.6
            _ if n >= 3520_u64 => n_ * (lg + lglg - 1.0 + (lglg - 2.1) / lg),
            // [1] Theorem A (ii)
            _ => n_ * (lg + lglg - 1.0),
        };

        let hi = match () {
            // [3] Proposition 6.6
            _ if n >= 688383_u64 => n_ * (lg + lglg - 1.0 + (lglg - 2.0) / lg),
            // [3] Lemma 6.5
            _ if n >= 178974_u64 => n_ * (lg + lglg - 1.0 + (lglg - 1.95) / lg),
            // [2] Theorem 1.8
            _ if n >= 39017_u64 => n_ * (lg + lglg - 0.9484),
            // [2] Theorem 1.7
            _ if n >= 27076_u64 => n_ * (lg + lglg - 1.0 + (lglg - 1.8) / lg),
            // [1] Theorem A (v)
            _ if n >= 15985_u64 => n_ * (lg + lglg - 0.9427),
            // [1] Theorem A (v)
            _ if n >= 13_u64 => n_ * (lg + lglg - 1.0 + 1.8 * lglg / lg),
            // [1] Theorem A (iv)
            _ => n_ * (lg + lglg),
        };
        (lo as u64, hi as u64)
    }
}

#[cfg(test)]
mod tests {
    extern crate primal;
    use self::primal::Sieve;

    #[test]
    fn prime_pi() {
        fn check(n: u64, pi: u64) {
            let (lo, hi) = super::prime_pi(n);
            assert!(lo <= pi && pi <= hi,
                    "found failing estimate at {}, should satisfy: {} <= {} <= {}",
                    n, lo, pi, hi)
        }
        let primes = Sieve::new(1_000_000);

        let mut last = 0;
        for (i, p) in primes.primes_from(0).enumerate() {
            for j in last..p {
                check(j as u64, i as u64);
            }
            last = p;
        }

        let sporadic = [
            (1, 4),
            (2, 25),
            (3, 168),
            (4, 1229),
            (5, 9592),
            (6, 78498),
            (7, 664579),
            (8, 5761455),
            (9, 50847534),
            (10, 455052511),
            (11, 4118054813),
            (12, 37607912018),
            (13, 346065536839),
            (14, 3204941750802),
            (15, 29844570422669),
            (16, 279238341033925),
            (17, 2623557157654233),
            ];
        for &(exponent, real) in sporadic.iter() {
            let n = 10u64.pow(exponent);
            check(n, real);
        }
    }

    #[test]
    fn nth_prime() {
        fn check(n: u64, p: u64) {
            let (lo, hi) = super::nth_prime(n);
            assert!(lo <= p && p <= hi,
                    "found failing estimate at {}, should satisfy: {} <= {} <= {}",
                    n, lo, p, hi);
        }
        let sieve = Sieve::new(1_000_000);

        for (i, p) in sieve.primes_from(0).enumerate() {
            let n = i as u64 + 1;
            check(n, p as u64);
        }

        let sporadic = [
            (0, 2),
            (1, 29),
            (2, 541),
            (3, 7919),
            (4, 104729),
            (5, 1299709),
            (6, 15485863),
            (7, 179424673),
            (8, 2038074743),
            (9, 22801763489),
            (10, 252097800623),
            (11, 2760727302517),
            (12, 29996224275833),
            (13, 323780508946331),
            (14, 3475385758524527),
            (15, 37124508045065437),
            ];

        for &(exponent, nth_prime) in sporadic.iter() {
            let n = 10u64.pow(exponent);
            check(n, nth_prime);
        }
    }
}
