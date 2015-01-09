use tables;
use std::num::Float;

/// Returns estimated bounds for π(*n*), the number of primes less
/// than or equal to `n`.
///
/// That is, if (*a*, *b*) = `estimate_prime_pi(n)`, *a* ≤ π(*n*) ≤
/// *b*. The bounds used are proved in [1] and [2, Théorème 1.10],
/// and are summarised in [2, pp. 14–15].
///
/// [1]: Barkley Rosser. "Explicit Bounds for Some Functions of Prime
/// Numbers". American Journal of Mathematics 63 (1):
/// 211–232. 1941. doi:[10.2307/2371291](http://dx.doi.org/10.2307/2371291).
///
///  [2]: Dusart, Pierre. ["Autour de la fonction qui compte le nombre
/// de nombres premiers."][pdf] PhD diss., Université de Limoges,
/// 1998.
///
/// [pdf]: http://www.unilim.fr/laco/theses/1998/T1998_01.html
pub fn estimate_prime_pi(n: u64) -> (u64, u64) {
    if n < tables::SMALL_PRIME_PI.len() as u64 {
        let x = tables::SMALL_PRIME_PI[n as usize] as u64;
        (x, x)
    } else {
        let n_ = n as f64;
        let lg = n_.ln();
        let inv_lg = 1.0 / lg;
        let n_lg = n_ * inv_lg;

        // numbers refer to parts of theorem 1.10 of [2].
        let lo = if n >= 32299 {
            // 6.
            n_lg * (1.0 + inv_lg * (1.0 + 1.8 * inv_lg))
        } else if n >= 5393 {
            // 5.
            n_ / (lg - 1.0)
        } else if n >= 599 {
            // 1.
            n_lg * (1.0 + inv_lg)
        } else {
            // [1]
            n_ / (lg + 2.0)
        };

        let hi = if n >= 13_220_000_000 {
            // 3.
            n_lg * (1.0 + 1.0992 * inv_lg)
        } else if n >= 355991 {
            // 7.
            n_lg * (1.0 + inv_lg * (1.0 + 2.51 * inv_lg))
        } else if n >= 60184 {
            // 4.
            n_ / (lg - 1.1)
        } else {
            // 2.
            n_lg * (1.0 + 1.2762 * inv_lg)
        };

        (lo as u64, hi as u64)
    }
}

/// Gives estimated bounds for *p<sub>n</sub>*, the `n`th prime number,
/// 1-indexed (i.e. *p<sub>1</sub>* = 2, *p<sub>2</sub>* = 3).
///
/// That is, if (<i>a</i>,<i>b</i>) = `estimate_nth_prime(n)`, *a* ≤
/// *p<sub>n</sub>* ≤ *b*. The bounds used are proved in [1] and [2,
/// Théorèmes 1.6–1.8], and are summarised in [2, pp. 14–15].
///
/// [1]: Massias, Jean-Pierre; Robin, Guy. ["Bornes effectives pour
/// certaines fonctions concernant les nombres
/// premiers."](http://eudml.org/doc/247826) Journal de théorie des
/// nombres de Bordeaux 8.1 (1996): 215-242.
///
/// [2]: Dusart, Pierre. ["Autour de la fonction qui compte le nombre
/// de nombres premiers."][pdf] PhD diss., Université de Limoges, 1998.
///
/// [pdf]: http://www.unilim.fr/laco/theses/1998/T1998_01.html
pub fn estimate_nth_prime(n: u64) -> (u64, u64) {
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

        let lo = lg + lglg - 1.0 + if n > 3 {//13196 {
            // [2] Theorem 1.6
            (lglg - 2.1) / lg
        } else {
            // [1] Theorem A (ii)
            0.0
        };

        let hi = lg + lglg + if n >= 39017 {
            // [2] Theorem 1.8
            -0.9484
        } else if n >= 27076 {
            // [2] Theorem 1.7
            -1.0 + (lglg - 1.8) / lg
        } else if n >= 15985 {
            // [1] Theorem A (v)
            -0.9427
        } else if n >= 13 {
            // [1] Theorem A (v)
            -1.0 + 1.8 * lglg / lg
        } else {
            // [1] Theorem A (iv)
            0.0
        };
        ((n_ * lo) as u64, (n_ * hi) as u64)
    }
}

#[cfg(test)]
mod tests {
    use std::num::Int;

    use Primes;
    use super::{estimate_prime_pi, estimate_nth_prime};

    #[test]
    fn prime_pi() {
        fn check(n: u64, pi: u64) {
            let (lo, hi) = estimate_prime_pi(n);
            assert!(lo <= pi && pi <= hi,
                    "found failing estimate at {}, should satisfy: {} <= {} <= {}",
                    n, lo, pi, hi)
        }
        let primes = Primes::sieve(1_000_000);

        let mut last = 0;
        for (i, p) in primes.primes().enumerate() {
            for j in range(last, p) {
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
            let (lo, hi) = estimate_nth_prime(n);
            assert!(lo <= p && p <= hi,
                    "found failing estimate at {}, should satisfy: {} <= {} <= {}",
                    n, lo, p, hi);
        }
        let sieve = Primes::sieve(1_000_000);

        for (i, p) in sieve.primes().enumerate() {
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
